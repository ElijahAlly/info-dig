import { NewStatementType, UpdateStatementType } from "@/interfaces/statements";
import { API_BASE_URL } from "./allApi";

// Fetch all statements
export const fetchAllStatements = async () => {
    try {
        const response = await fetch(`${API_BASE_URL}/statements`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                // 'Authorization': `Bearer ${yourAuthToken}`,
            },
        });

        if (!response.ok) {
            throw new Error(`Error: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Error fetching all statements:', error);
        throw error;
    }
};

// Fetch a single statement by ID
export const fetchStatementById = async (slug: string) => {
    try {
        const response = await fetch(`${API_BASE_URL}/statements/${slug}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                // Include authorization headers if needed
                // 'Authorization': `Bearer ${yourAuthToken}`,
            },
        });

        if (!response.ok) {
            throw new Error(`Error: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error(`Error fetching statement with slug: ${slug}:`, error);
        throw error;
    }
};

// Create a single statement
export const createStatement = async (statement: NewStatementType) => {
    try {
        const response = await fetch(`${API_BASE_URL}/statements`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                // Include authorization headers if needed
                // 'Authorization': `Bearer ${yourAuthToken}`,
            },
            body: JSON.stringify(statement),
        });

        if (!response.ok) {
            throw new Error(`Error: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Error reating statement');
        throw error;
    }
};

// Update a single statement
export const updateStatement = async (statement: UpdateStatementType) => {
    // console.log('statement', statement);
    try {
        const response = await fetch(`${API_BASE_URL}/statements/${statement.statement_id}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
                // Include authorization headers if needed
                // 'Authorization': `Bearer ${yourAuthToken}`,
            },
            body: JSON.stringify(statement),
        });

        if (!response.ok) {
            throw new Error(`Error: ${response.status}`);
        }

        return await response.json();
    } catch (error) {
        console.error('Error reating statement');
        throw error;
    }
};
