<script lang="ts">
    interface Props {
        activeTab: string;
        onTabChange: (tab: string) => void;
    }

    let { activeTab, onTabChange }: Props = $props();
</script>

<!-- Tab Navigation -->
<div class="tab-navigation">
    <button
        class="tab-button {activeTab === 'api' ? 'active' : ''}"
        onclick={() => onTabChange("api")}
    >
        <i class="bi bi-key"></i>
        API Configuration
    </button>
    <button
        class="tab-button {activeTab === 'models' ? 'active' : ''}"
        onclick={() => onTabChange("models")}
    >
        <i class="bi bi-cpu"></i>
        Model Management
    </button>
    <button
        class="tab-button {activeTab === 'behavior' ? 'active' : ''}"
        onclick={() => onTabChange("behavior")}
    >
        <i class="bi bi-sliders"></i>
        App Behavior
    </button>
    <button
        class="tab-button {activeTab === 'about' ? 'active' : ''}"
        onclick={() => onTabChange("about")}
    >
        <i class="bi bi-info-circle"></i>
        About
    </button>
</div>

<style>
    .tab-navigation {
        display: flex;
        border-bottom: 1px solid #e0e0e0;
        background: #f8f9fa;
    }

    .tab-button {
        flex: 1;
        padding: 12px 16px;
        border: none;
        background: none;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        font-size: 14px;
        font-weight: 500;
        color: #666;
        transition: all 0.2s;
        border-bottom: 2px solid transparent;
    }

    .tab-button:hover {
        background: #e9ecef;
        color: #333;
    }

    .tab-button.active {
        color: #379df1;
        border-bottom-color: #379df1;
        background: white;
    }

    @media (prefers-color-scheme: dark) {
        .tab-navigation {
            border-color: #444;
            background: #383838;
        }

        .tab-button {
            color: #ccc;
        }

        .tab-button:hover {
            background: #444;
            color: #f6f6f6;
        }

        .tab-button.active {
            color: #379df1;
            background: #2d2d2d;
        }
    }
</style>
