import { defineStore } from 'pinia';

export const useModalsStore = defineStore('modals', {
    state: () => ({
        createStatements: {
            isVisible: false,
            content: '',
            context: '',
            links: [{ url: '', name: '' }]
        }
    }),
    getters: {
        getCreateStatements: (state) => state.createStatements,
    },
    actions: {
        showCreateStatements() {
            // console.log('isVisible', this.createStatements.isVisible);
            this.createStatements.isVisible = true;
        },
        hideCreateStatements() {
            // console.log('isVisible', this.createStatements.isVisible);
            this.createStatements.isVisible = false;
        },
        addLink() {
            console.log('addLink', this.createStatements.links);
            this.createStatements.links.push({ url: '', name: '' })
            console.log('addLink', this.createStatements.links);
        },
        removeLink(index: number) {
            const newLinks = [];
            for (let i = 0; i < this.createStatements.links.length; i++) {
                if (index !== i) {
                    newLinks.push(this.createStatements.links[i]);
                }
            }
            this.createStatements.links = newLinks; 
        }
    },
});
