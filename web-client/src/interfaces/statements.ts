export interface StatementType {
    statement_id: number;
    user_id: number;
    slug: string;
    content: string;
    context: string;
    public_rating: string;
    our_rating: string;
    created_at: string;
    updated_at: string;
}

export interface NewStatementType {
    user_id: number;
    slug: string;
    content: string;
    context: string;
}
