import { StatementType } from '@/interfaces/statements';
import { defineStore } from 'pinia';

export const useStatementsStore = defineStore('statements', {
    state: () => ({
        statements: [] as StatementType[],
        selectedStatement: {
            statement_id: 0,
            user_id: 0,
            context: '',
            content: '',
            slug: '',
            public_rating: '',
            our_rating: '',
            links: [],
            created_at: '',
            updated_at: ''
        } as StatementType,
        sortColumnsByPublicView: false as boolean,
        sortColumnsByOurTeamView: false as boolean
    }),
    getters: {
        getAllStatements: (state) => state.statements,
        getSelectedStatement: (state) => state.selectedStatement,
    },
    // actions: {

    // },
});
