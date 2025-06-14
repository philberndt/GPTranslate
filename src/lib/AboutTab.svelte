<script lang="ts">
    interface Props {
        version: string;
    }

    let { version }: Props = $props();
</script>

<!-- About Section -->
<section class="settings-section">
    <h3><i class="bi bi-info-circle"></i>About</h3>

    <div class="about-content">
        <div class="about-item">
            <strong>Developer:</strong> Phil Berndt
        </div>
        <div class="about-item">
            <strong>Email:</strong> phil@berndt.no
        </div>
        <div class="about-item">
            <strong>Website:</strong>
            <a
                href="https://gptranslate.berndt.no"
                target="_blank"
                rel="noopener noreferrer"
            >
                https://gptranslate.berndt.no
            </a>
        </div>
        <div class="about-item">
            <strong>Version:</strong>
            {version}
        </div>
    </div>
</section>

<style>
    .settings-section {
        margin: 24px 0;
        min-width: 0;
    }

    .settings-section h3 {
        font-size: 1.1rem;
        color: #333;
        margin: 0 0 16px 0;
        display: flex;
        align-items: center;
        gap: 8px;
        padding-bottom: 8px;
        border-bottom: 1px solid #e0e0e0;
    }

    .about-content {
        background: #f8f9fa;
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        padding: 16px;
    }

    .about-item {
        margin-bottom: 8px;
        font-size: 14px;
        line-height: 1.5;
    }

    .about-item:last-child {
        margin-bottom: 0;
    }

    .about-item strong {
        color: #333;
        min-width: 80px;
        display: inline-block;
    }

    .about-item a {
        color: #379df1;
        text-decoration: none;
        transition: color 0.2s;
    }

    .about-item a:hover {
        color: #2980e6;
        text-decoration: underline;
    }

    /* Dark mode */
    @media (prefers-color-scheme: dark) {
        .settings-section h3 {
            color: #f6f6f6;
            border-color: #444;
        }

        .about-content {
            background: #383838;
            border-color: #444;
        }

        .about-item strong {
            color: #f6f6f6;
        }

        .about-item a {
            color: #379df1;
        }

        .about-item a:hover {
            color: #2980e6;
        }
    }
</style>
