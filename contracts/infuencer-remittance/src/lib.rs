#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, String, Vec
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Campaigns,
    Proposals,
    Token,
}

#[contracttype]
#[derive(Clone)]
pub struct Campaign {
    id: u32,
    organization: Address,
    title: String,
    budget: i128,
    status: CampaignStatus,
    deadline: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    id: u32,
    campaign_id: u32,
    influencer: Address,
    amount: i128,
    timeline: u64,
    status: ProposalStatus,
    deliverables: Vec<String>,
}

#[contracttype]
#[derive(Clone)]
pub enum CampaignStatus {
    Active,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Completed,
    Rejected,
}

#[contract]
pub struct InfluencerPayment;

#[contractimpl]
impl InfluencerPayment {
    pub fn init(env: Env, token: Address) {
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::Campaigns, &Vec::<Campaign>::new(&env));
        env.storage().persistent().set(&DataKey::Proposals, &Vec::<Proposal>::new(&env));
    }

    // Organization creates a new campaign
    pub fn create_campaign(
        env: Env,
        organization: Address,
        title: String,
        budget: i128,
        deadline: u64,
    ) -> u32 {
        organization.require_auth();

        let campaign = Campaign {
            id: env.ledger().sequence() as u32,
            organization,
            title,
            budget,
            status: CampaignStatus::Active,
            deadline,
        };

        let mut campaigns: Vec<Campaign> = env.storage().persistent().get(&DataKey::Campaigns).unwrap();
        campaigns.push_back(campaign.clone());
        env.storage().persistent().set(&DataKey::Campaigns, &campaigns);

        campaign.id
    }

    // Influencer submits a proposal
    pub fn submit_proposal(
        env: Env,
        campaign_id: u32,
        influencer: Address,
        amount: i128,
        timeline: u64,
        deliverables: Vec<String>,
    ) -> u32 {
        influencer.require_auth();

        // Verify campaign exists and is active
        let campaign = Self::get_campaign(&env, campaign_id);
        assert!(
            matches!(campaign.status, CampaignStatus::Active),
            "Campaign is not active"
        );
        assert!(amount <= campaign.budget, "Amount exceeds campaign budget");

        let proposal = Proposal {
            id: env.ledger().sequence() as u32,
            campaign_id,
            influencer,
            amount,
            timeline,
            status: ProposalStatus::Pending,
            deliverables,
        };

        let mut proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        proposals.push_back(proposal.clone());
        env.storage().persistent().set(&DataKey::Proposals, &proposals);

        proposal.id
    }

    // Organization approves a proposal and locks the payment
    pub fn approve_proposal(env: Env, proposal_id: u32) {
        let proposal = Self::get_proposal(&env, proposal_id);
        let campaign = Self::get_campaign(&env, proposal.campaign_id);
        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);
        
        campaign.organization.require_auth();
        
        // Lock the USDC payment
        token_client.transfer(
            &campaign.organization,
            &env.current_contract_address(),
            &proposal.amount,
        );

        // Update proposal status
        let mut proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let index = proposals
            .iter()
            .position(|p| p.id == proposal_id)
            .expect("Proposal not found") as u32;  // Convert usize to u32
        
        proposals.set(
            index,
            Proposal {
                status: ProposalStatus::Approved,
                ..proposal
            },
        );
        env.storage().persistent().set(&DataKey::Proposals, &proposals);
    }

    // Organization confirms task completion and releases payment
    pub fn complete_task(env: Env, proposal_id: u32) {
        let proposal = Self::get_proposal(&env, proposal_id);
        let campaign = Self::get_campaign(&env, proposal.campaign_id);
        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);
        
        campaign.organization.require_auth();
        
        assert!(
            matches!(proposal.status, ProposalStatus::Approved),
            "Proposal not in approved state"
        );

        // Transfer locked USDC to influencer
        token_client.transfer(
            &env.current_contract_address(),
            &proposal.influencer,
            &proposal.amount,
        );

        // Update proposal status
        let mut proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let index = proposals
            .iter()
            .position(|p| p.id == proposal_id)
            .expect("Proposal not found") as u32;
        
        proposals.set(
            index,
            Proposal {
                status: ProposalStatus::Completed,
                ..proposal
            },
        );
        env.storage().persistent().set(&DataKey::Proposals, &proposals);

        // Update campaign status if needed
        if Self::count_remaining_proposals(&env, proposal.campaign_id) == 0 {
            let mut campaigns: Vec<Campaign> = env.storage().persistent().get(&DataKey::Campaigns).unwrap();
            let campaign_index = campaigns
                .iter()
                .position(|c| c.id == proposal.campaign_id)
                .expect("Campaign not found") as u32;
            
            campaigns.set(
                campaign_index,
                Campaign {
                    status: CampaignStatus::Completed,
                    ..campaign
                },
            );
            env.storage().persistent().set(&DataKey::Campaigns, &campaigns);
        }
    }

    // Helper functions
    fn get_campaign(env: &Env, campaign_id: u32) -> Campaign {
        let campaigns: Vec<Campaign> = env.storage().persistent().get(&DataKey::Campaigns).unwrap();
        campaigns
            .iter()
            .find(|c| c.id == campaign_id)
            .expect("Campaign not found")
            .clone()
    }

    fn get_proposal(env: &Env, proposal_id: u32) -> Proposal {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .expect("Proposal not found")
            .clone()
    }

    // View functions
    pub fn get_campaign_proposals(env: Env, campaign_id: u32) -> Vec<Proposal> {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let mut filtered = Vec::new(&env);
        
        for i in 0..proposals.len() {
            let proposal = proposals.get(i).unwrap();
            if proposal.campaign_id == campaign_id {
                filtered.push_back(proposal);
            }
        }
        filtered
    }

    pub fn get_influencer_proposals(env: Env, influencer: Address) -> Vec<Proposal> {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let mut filtered = Vec::new(&env);
        
        for i in 0..proposals.len() {
            let proposal = proposals.get(i).unwrap();
            if proposal.influencer == influencer {
                filtered.push_back(proposal);
            }
        }
        filtered
    }

    pub fn get_active_campaigns(env: Env) -> Vec<Campaign> {
        let campaigns: Vec<Campaign> = env.storage().persistent().get(&DataKey::Campaigns).unwrap();
        let mut active_campaigns = Vec::new(&env);
        
        for i in 0..campaigns.len() {
            let campaign = campaigns.get(i).unwrap();
            if matches!(campaign.status, CampaignStatus::Active) {
                active_campaigns.push_back(campaign);
            }
        }
        active_campaigns
    }


    fn count_remaining_proposals(env: &Env, campaign_id: u32) -> u32 {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let mut count = 0;
        
        for i in 0..proposals.len() {
            let proposal = proposals.get(i).unwrap();
            if proposal.campaign_id == campaign_id && !matches!(proposal.status, ProposalStatus::Completed) {
                count += 1;
            }
        }
        count
    }
}