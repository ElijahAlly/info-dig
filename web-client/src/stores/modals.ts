import { defineStore } from 'pinia';
import { StatementType } from '@/interfaces/statements';

export const useModalsStore = defineStore('modals', {
    state: () => ({
        createStatement: {
            isVisible: false,
            content: '',
            context: '',
            links: [{ url: '', name: '' }]
        },
        editStatement: {
            isVisible: false,
            statement_id: 0,
            content: '',
            context: '',
            links: [{ url: '', name: '' }]
        }
    }),
    getters: {
        getCreateStatement: (state) => state.createStatement,
        getEditStatement: (state) => state.editStatement,
    },
    actions: {
        // * Create
        showCreateStatements() {
            this.createStatement.isVisible = true;
        },
        hideCreateStatements() {
            this.createStatement.isVisible = false;
        },
        addCreateLink() {
            this.createStatement.links.push({ url: '', name: '' })
        },
        removeCreateLink(index: number) {
            const newLinks = [];
            for (let i = 0; i < this.createStatement.links.length; i++) {
                if (index !== i) {
                    newLinks.push(this.createStatement.links[i]);
                }
            }
            this.createStatement.links = newLinks;
        },
        // * Edit
        showEditStatements(statement: StatementType) {
            this.editStatement.statement_id = statement.statement_id;
            this.editStatement.content = statement.content;
            this.editStatement.context = statement.context;
            if (statement.links.length) {
                this.editStatement.links = statement.links;
            }
            this.editStatement.isVisible = true;
        },
        hideEditStatements() {
            this.editStatement.isVisible = false;
        },
        addEditLink() {
            this.editStatement.links.push({ url: '', name: '' })
        },
        removeEditLink(index: number) {
            const newLinks = [];
            for (let i = 0; i < this.editStatement.links.length; i++) {
                if (index !== i) {
                    newLinks.push(this.editStatement.links[i]);
                }
            }
            this.editStatement.links = newLinks;
        }
    },
});
