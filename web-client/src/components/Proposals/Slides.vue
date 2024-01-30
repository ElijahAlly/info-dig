<template>
    <div class="slides-cont">
        <div v-if="proposals && proposals.length" class="slides">
            <div class="selected-proposal">
                <div class="info-box">
                    <div class="img-cont">
                        <img
                        src="@/assets/logos/3d-approved-proposal-icon.png"
                        alt="proposal-logo"
                        :height="imgHeight"
                        :width="imgWidth"
                        @mouseenter="handleImgHover"
                        @mouseleave="handleImgExit"
                        class="progress-img"
                    >
                    </div>
                    <!-- * Options: Project-Proposal-icon.png | 3d-approved-proposal-icon.png -->
                    <div class="progress-bar-container">
                        <div class="bar-section yeas-bar" :style="{ width: yeasWidth + '%' }">
                            <span class="progress-label">
                                Yeas
                            </span>
                            <span class="progress-value">
                                {{ yeasCount }}
                            </span>
                        </div>
                        <div class="bar-section nays-bar" :style="{ width: naysWidth + '%' }">
                            <span class="progress-label">Nays</span>
                            <span class="progress-value">{{ naysCount }}</span>
                        </div>
                    </div>
                </div>
                <div class="details">
                    <h2>{{ selectedProposal.title }}</h2>
                    <p>{{ selectedProposal.description }}</p>
                    <p>{{ selectedProposal.status }}</p>
                </div>
            </div>
            <div class="backlog">
                <div
                    v-for="(proposal, i) in proposals"
                    :key="i"
                    :class="['item', proposal.proposal_id === selectedProposal.proposal_id ? 'highlighted' : '']"
                    @click="() => handleProposalClick(proposal.proposal_id)"
                >
                    <h2>{{ proposal.title }}</h2>
                    <p>{{ proposal.description }}</p>
                    <p>{{ proposal.status }}</p>
                </div>
            </div>
        </div>
        <div v-else>Loading proposals...</div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ProposalType } from '@/interfaces/proposals';
import { useProposalsStore } from '@/stores/proposals';
import { fetchAllProposals } from '@/api-utils/proposalsApi';

export default defineComponent({
    name: 'Slides',
    mounted() {
        this.loadAllProposals();
        if (this.selectedProposal) {
            this.animateCount('naysCount', this.selectedProposal.nays);
            this.animateCount('yeasCount', this.selectedProposal.yeas);
        }
    },
    methods: {
        async loadAllProposals() {
            try {
                const proposals: ProposalType[] = await fetchAllProposals();
                useProposalsStore().$patch((state) => {
                    const proposalIds = new Set(state.proposals.map(p => p.proposal_id));
                    proposals.forEach((newProposal) => {
                        if (!proposalIds.has(newProposal.proposal_id)) {
                            state.proposals.push(newProposal);
                        }
                    });
                })
            } catch (error) {
                console.error('Failed to load proposals:', error);
            }
        },
        handleProposalClick(clickedId: number) {
            this.naysCount = 0;
            this.yeasCount = 0;
            const foundProposal = this.proposals.find(proposal => proposal.proposal_id === clickedId);
            foundProposal && useProposalsStore().$patch({ selectedProposal: foundProposal });
            this.animateCount('naysCount', foundProposal?.nays || 0);
            this.animateCount('yeasCount', foundProposal?.yeas || 0);
        },
        animateCount(prop: string, target: number) {
            const speed = 12; // Adjust this to control the speed of the animation
            const step = target > 100 ? target > 1000 ? target / 200 : target / 100 : target / target; // Number of increments needed
            if (prop !== 'naysCount' && prop !== 'yeasCount') return;

            const counter = setInterval(() => {
                this[prop] += step;

                this[prop] = Math.round(this[prop]);

                if (this[prop] >= target) {
                    clearInterval(counter);
                    this[prop] = target;
                }
            }, speed);
        },
        handleImgHover() {
            this.imgHeight = 210;
            this.imgWidth = 210;
        },
        handleImgExit() {
            this.imgHeight = 200;
            this.imgWidth = 200;
        }
    },
    data() {
        return {
            naysCount: 0,
            yeasCount: 0,
            imgHeight: 200,
            imgWidth: 200,
        }
    },
    computed: {
        proposals() {
            return useProposalsStore().getAllProposals;
        },
        selectedProposal() {
            if (!useProposalsStore().getSelectedProposal?.proposal_id) return this.proposals[0];
            return useProposalsStore().getSelectedProposal;
        },
        totalVotes() {
            return this.yeasCount + this.naysCount;
        },
        yeasWidth() {
            const totalVotes = this.yeasCount + this.naysCount;
            const rawWidth = totalVotes ? (this.yeasCount / totalVotes) * 100 : 0;
            return Math.max(rawWidth, 10); /* Ensure a minimum width */
        },
        naysWidth() {
            return Math.max(100 - this.yeasWidth, 10); /* Ensure a minimum width */
        }
        // yeasHeight() {
        //     const totalVotes = this.yeasCount + this.naysCount;
        //     return totalVotes ? (this.yeasCount / totalVotes) * 100 : 0;
        // },
        // naysHeight() {
        //     return 100 - this.yeasHeight;
        // }
    },
})
</script>

<style scoped lang="scss">
.slides-cont {
    display: flex;
    justify-content: space-evenly;
    margin-top: 3vh;
    overflow: hidden;
    max-height: 81vh;
    width: 90vw;

    .slides {
        display: flex;
        overflow: hidden;

        .selected-proposal {
            display: flex;
            flex-direction: column;
            margin-right: 3vw;
            width: 21vw;
            min-width: 200px;
            max-width: 21vw;
            padding-bottom: 12vw;

            .info-box {
                padding: 20px;
                background-color: #392525;
                border-radius: 6px;

                .img-cont {
                    height: 210px;
                    width: 100%;
                    display: flex;
                    align-items: center;
                    justify-content: center;

                    .progress-img {
                        &:hover {
                            rotate: -21deg 21 3 1;
                        }
                    }
                }

                .progress-bar-container {
                    height: 42px; /* Adjust as needed */
                    width: 100%; /* Adjust as needed */
                    position: relative;
                    border-radius: 10px;
                    overflow: hidden;
                    display: flex;
                    flex-direction: row;
                }

                .bar-section {
                    height: 100%;
                    position: relative;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 48px;


                    .progress-label {
                        color: white;
                        font-weight: lighter;
                        // position: absolute; /* Use absolute positioning */
                        // left: 50%; /* Center the label */
                        // transform: translateX(-50%);
                    }

                    .progress-value {
                        color: white;
                        font-weight: bold; 
                    }
                }

                .yeas-bar {
                    background-color: #0fce76; /* Color for yeas */
                }

                .nays-bar {
                    background-color: #c26769; /* Color for nays */
                }

            }

            .details {
                overflow-y: auto;
                border-bottom-left-radius: 6px;
                border-bottom-right-radius: 6px;
                box-shadow: 0px -12px 9px -12px inset #392525;

               &::-webkit-scrollbar {
                    display: none;
                } 
            }

        }

        .backlog {
            display: flex;
            flex-direction: column;
            overflow-y: auto;
            padding-bottom: 21vh;
            min-width: 60vw;

            &::-webkit-scrollbar {
                display: none;
            }
            
            .item {
                border: 2px solid #392525; // #392525
                border-radius: 6px;
                margin-bottom: 6px;
                cursor: pointer;
                color: #392525;

                &:hover {
                    border-color: #c26769;
                }
            }
            
            .highlighted {
                border-color: #d1c13f;
            }
        }
    }
}
</style>
