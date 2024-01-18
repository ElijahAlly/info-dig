import { StatementType } from '@/interfaces/statements';
import { defineStore } from 'pinia';

export const useStatementsStore = defineStore('statements', {
    state: () => ({
        statements: [] as StatementType[],
        orderColumnsByPublicView: false as boolean
    }),
    getters: {
        getAllStatementsFromState: (state) => state.statements,
    },
    // actions: {

    // },
});
