import { StatementType } from '@/interfaces/statements';
import { defineStore } from 'pinia';

export const useStatementsStore = defineStore('statements', {
    state: () => ({
        statements: [] as StatementType[],
        name: 'Eduardo'
    }),
    getters: {
        getAllStatementsFromState: (state) => state.statements,
    },
    // actions: {
    //     increment() {
    //         this.count++
    //     },
    // },
});
