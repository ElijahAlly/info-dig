import { ProposalType } from '@/interfaces/proposals';
import { defineStore } from 'pinia';

export const useProposalsStore = defineStore('proposals', {
    state: () => ({
        proposals: [] as ProposalType[],
        selectedProposal: {} as ProposalType,
        sortColumnsByPublicView: false as boolean,
        sortColumnsByOurTeamView: false as boolean
    }),
    getters: {
        getAllProposals: (state) => state.proposals,
        getSelectedProposal: (state) => state.selectedProposal,
    },
    // actions: {

    // },
});
