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

        // Get existing campaigns or create new vector if none exist
        let mut campaigns: Vec<Campaign> = env
            .storage()
            .persistent()
            .get(&DataKey::Campaigns)
            .unwrap_or_else(|| Vec::new(&env));

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

        // Get existing proposals or create new vector if none exist
        let mut proposals: Vec<Proposal> = env
            .storage()
            .persistent()
            .get(&DataKey::Proposals)
            .unwrap_or_else(|| Vec::new(&env));

        proposals.push_back(proposal.clone());
        env.storage().persistent().set(&DataKey::Proposals, &proposals);

        proposal.id
    }

    // Organization approves a proposal and locks the payment
   pub fn approve_proposal(env: &Env, proposal_id: u32) {
    // Get current state
        let proposal = Self::get_proposal(env, proposal_id);
        let campaign = Self::get_campaign(env, proposal.campaign_id);
        let token: Address = env.storage().persistent().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(env, &token);
        
        // Require authorization from campaign organization
        campaign.organization.require_auth();
        
        // Lock the USDC payment
        token_client.transfer(
            &campaign.organization,
            &env.current_contract_address(),
            &proposal.amount
        );

        // Update proposal status
        let mut proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        let mut updated_proposals: Vec<Proposal> = Vec::new(env);
        
        // Create new vector with updated proposal
        for p in proposals.iter() {
            if p.id == proposal_id {
                let updated_proposal = Proposal {
                    id: p.id,
                    campaign_id: p.campaign_id,
                    influencer: p.influencer.clone(),
                    amount: p.amount,
                    status: ProposalStatus::Approved,
                    deliverables: proposal.deliverables.clone(),
                    timeline: proposal.timeline
                };
                updated_proposals.push_back(updated_proposal);
            } else {
                updated_proposals.push_back(p.clone());
            }
        }
        
        // Save the updated proposals
        env.storage().persistent().set(&DataKey::Proposals, &updated_proposals);
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
    pub fn get_campaign(env: &Env, campaign_id: u32) -> Campaign {
        let campaigns: Vec<Campaign> = env
            .storage()
            .persistent()
            .get(&DataKey::Campaigns)
            .unwrap_or_else(|| Vec::new(env));

        campaigns
            .iter()
            .find(|c| c.id == campaign_id)
            .expect("Campaign not found")
            .clone()
    }

    pub fn get_proposal(env: &Env, proposal_id: u32) -> Proposal {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .expect("Proposal not found")
            .clone()
    }

    // View functions
    pub fn get_campaign_proposals(env: &Env, campaign_id: u32) -> Vec<Proposal> {
        let proposals: Vec<Proposal> = env.storage().persistent().get(&DataKey::Proposals).unwrap();
        // Create a new vector with the environment reference
        let mut filtered: Vec<Proposal> = Vec::new(env);
        
        for proposal in proposals.iter() {  // Use iter() for more idiomatic iteration
            if proposal.campaign_id == campaign_id {
                filtered.push_back(proposal.clone());  // Clone the proposal when pushing
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

    pub fn get_organization_campaigns(env: &Env, organization: Address) -> Vec<Campaign> {
    // Get campaigns from storage with error handling
        let campaigns: Vec<Campaign> = env
            .storage()
            .persistent()
            .get(&DataKey::Campaigns)
            .unwrap_or_else(|| Vec::new(env));

        let mut filtered: Vec<Campaign> = Vec::new(env);

        // Filter for both organization and active status in one pass
        for campaign in campaigns.iter() {
            if campaign.organization == organization && matches!(campaign.status, CampaignStatus::Active) {
                filtered.push_back(campaign.clone());
            }
        }

        filtered
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