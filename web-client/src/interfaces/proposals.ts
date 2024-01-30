export interface ProposalType {
    proposal_id: number,
    user_id: number,
    slug: string,
    title: string,
    description: string,
    status: string,
    yeas: number,
    nays: number,
    created_at: string,
    updated_at: string
}
