// src/services/contractService.ts
import { kit, loadedPublicKey } from "../stellar-wallets-kit";
import infuencer_remittance from "../contracts/infuencer_remittance";
export interface Campaign {
	id: number;
	organization: string;
	title: string;
	budget: bigint;
	status: "Active" | "Completed" | "Cancelled";
	deadline: number;
}

export interface Proposal {
	id: number;
	campaign_id: number;
	influencer: string;
	amount: bigint;
	timeline: number;
	status: "Pending" | "Approved" | "Completed" | "Rejected";
	deliverables: string[];
}

export class ContractService {
	private userPublicKey!: string;

	constructor() {
		const publicKey = loadedPublicKey();
		if (!publicKey) {
			alert("Please connect your wallet first");
			return;
		}
		this.userPublicKey = publicKey;
		infuencer_remittance.options.publicKey = publicKey;
	}

	async submitProposal(
		campaignId: number,
		amount: bigint,
		timeline: bigint,
		deliverables: string[]
	): Promise<number> {
		try {
			const tx = await infuencer_remittance.submit_proposal({
				campaign_id: campaignId,
				influencer: this.userPublicKey,
				amount,
				timeline,
				deliverables,
			});
			const { result } = await tx.signAndSend({
				signTransaction: async (xdr) => {
					const { signedTxXdr } = await kit.signTransaction(xdr);
					return signedTxXdr;
				},
			});
			return Number(result);
		} catch (error) {
			console.error("Error submitting proposal:", error);
			throw error;
		}
	}

	async createCampaign(
		title: string,
		budget: bigint,
		deadline: bigint
	): Promise<number> {
		try {
			const tx = await infuencer_remittance.create_campaign({
				organization: this.userPublicKey,
				title,
				budget,
				deadline,
			});
			const { result } = await tx.signAndSend({
				signTransaction: async (xdr) => {
					const { signedTxXdr } = await kit.signTransaction(xdr);
					return signedTxXdr;
				},
			});
			return Number(result);
		} catch (error) {
			console.error("Error submitting proposal:", error);
			throw error;
		}
	}

	async approveProposal(proposalId: string): Promise<number> {
		try {
			const tx = await infuencer_remittance.approve_proposal({
				proposal_id: proposalId,
			});
			const { result } = await tx.signAndSend({
				signTransaction: async (xdr) => {
					const { signedTxXdr } = await kit.signTransaction(xdr);
					return signedTxXdr;
				},
			});
			return Number(result);
		} catch (error) {
			console.error("Error submitting proposal:", error);
			throw error;
		}
	}

	async getInfluencerProposals(): Promise<Proposal[]> {
		try {
			const result =
				await infuencer_remittance.get_influencer_proposals({
					influencer: this.userPublicKey,
				});
			return result as unknown as Proposal[];
		} catch (error) {
			console.error("Error fetching proposals:", error);
			throw error;
		}
	}

	async getCampaignProposals(campaignId: number): Promise<Proposal[]> {
		try {
			const result = await infuencer_remittance.get_campaign_proposals(
				{ campaign_id: campaignId }
			);
			return result.result;
		} catch (error) {
			console.error("Error fetching campaign proposals:", error);
			throw error;
		}
	}

	async getCampaign(campaignId: number): Promise<Campaign> {
		try {
			const result = await infuencer_remittance.get_campaign({
				campaign_id: campaignId,
			});
			return result.result;
		} catch (error) {
			console.error("Error fetching active campaigns:", error);
			throw error;
		}
	}

	async getActiveCampaigns(): Promise<Campaign[]> {
		try {
			const result = await infuencer_remittance.get_active_campaigns();
			return result.result;
		} catch (error) {
			console.error("Error fetching active campaigns:", error);
			throw error;
		}
	}

	async getActiveCampaignsForOrg(): Promise<Campaign[]> {
		try {
			const result =
				await infuencer_remittance.get_organization_campaigns({
					organization: this.userPublicKey,
				});
			return result.result;
		} catch (error) {
			console.error("Error fetching active campaigns:", error);
			throw error;
		}
	}
}
