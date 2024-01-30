// import { NewProposalType, UpdateProposalType } from "@/interfaces/proposals";
import { API_BASE_URL } from "./allApi";

// Fetch all proposals
export const fetchAllProposals = async () => {
    try {
        const response = await fetch(`${API_BASE_URL}/proposals`, {
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
        console.error('Error fetching all proposals:', error);
        throw error;
    }
};

// Fetch a single proposal by ID
export const fetchProposalById = async (slug: string) => {
    try {
        const response = await fetch(`${API_BASE_URL}/proposals/${slug}`, {
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
        console.error(`Error fetching proposal with slug: ${slug}:`, error);
        throw error;
    }
};

// // Create a single proposal
// export const createProposal = async (proposal: NewProposalType) => {
//     try {
//         const response = await fetch(`${API_BASE_URL}/proposals`, {
//             method: 'POST',
//             headers: {
//                 'Content-Type': 'application/json',
//                 // Include authorization headers if needed
//                 // 'Authorization': `Bearer ${yourAuthToken}`,
//             },
//             body: JSON.stringify(proposal),
//         });

//         if (!response.ok) {
//             throw new Error(`Error: ${response.status}`);
//         }

//         return await response.json();
//     } catch (error) {
//         console.error('Error reating proposal');
//         throw error;
//     }
// };

// // Update a single proposal
// export const updateProposal = async (proposal: UpdateProposalType) => {
//     // console.log('proposal', proposal);
//     try {
//         const response = await fetch(`${API_BASE_URL}/proposals/${proposal.proposal_id}`, {
//             method: 'PUT',
//             headers: {
//                 'Content-Type': 'application/json',
//                 // Include authorization headers if needed
//                 // 'Authorization': `Bearer ${yourAuthToken}`,
//             },
//             body: JSON.stringify(proposal),
//         });

//         if (!response.ok) {
//             throw new Error(`Error: ${response.status}`);
//         }

//         return await response.json();
//     } catch (error) {
//         console.error('Error reating proposal');
//         throw error;
//     }
// };
