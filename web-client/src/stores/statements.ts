import { StatementType } from '@/interfaces/statements';
import { defineStore } from 'pinia';

export const useStatementsStore = defineStore('statements', {
    state: () => ({
        statements: [] as StatementType[],
        sortColumnsByPublicView: false as boolean,
        sortColumnsByOurTeamView: false as boolean
    }),
    getters: {
        getAllStatementsFromState: (state) => state.statements,
    },
    // actions: {

    // },
});
