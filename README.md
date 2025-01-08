<p align="center">
<img src="imgs/banner.png" alt="Fabelis Banner" width="100%" />
</p>
<p align="center">
<h1 align="center">FABELIS.AI Agent</h1>
<p align="center">
A framework for AI-driven storytelling and content generation
</p>

<p align="center">
<a href="https://github.com/fabelis/agent"><img src="https://img.shields.io/github/stars/fabelis/agent?style=social" alt="stars - rig" /></a>
&nbsp;
<a href="https://docs.fabelis.ai"><img src="https://img.shields.io/badge/🤖 docs-Fabelis-blue.svg" /></a>
&nbsp;
</p>

✨ If you like Fabelis, please consider starring the repo!

## What is Fabelis?

Fabelis is a Rust-based framework that enables AI agents to generate interactive storytelling content and blog posts. Originally designed as an AI agent writing its own story through blog posts, it has evolved into a comprehensive framework for automated content generation and publishing.

## High-level Features

- Full support for AI-driven content generation and storytelling
- Configurable AI agents with customizable personalities
- Automated blog post generation and publishing workflow
- Extensible architecture supporting multiple providers and databases
- Added custom memory store built on **local** embedding methods and in memory Vector DB

## Quick Start

### Step 1: Clone the Repository
```bash
git clone git@github.com:fabelis/agent.git
```

### Step 2: Configure Your Agent
Create a `config.json` in the root directory:
```json
{
    "clients": {
        "api": {
            "port": 3000
        },
        "cli": true,
        "storytelling": {
            "port": 3001,
            "paragraph_count": [
                3,
                7
            ]
        },
        "twitter": {
            "post_delay": [
                10,
                20
            ],
            "reply_delay": [
                10,
                20
            ],
            "search_delay": 1,
            "delay": 0,
            "debug": true
        }
    },
    "provider": "anthropic",
    "embed_provider": "local",
    "db": "local"
}
```
**ONLY Include Agents you want to run**
*CLI cannot run at the same time as any other agent*

### Step 3: Environment Setup
Create a `.env` file based on `.env.example` and add necessary credentials:
```env
ANTHROPIC_API_KEY="your_key_here"
ANTHROPIC_COMPLETION_MODEL="claude-3-5-sonnet-latest"
```

### Step 4: Create Your Character
Place your character configuration in the `characters` folder. Example usage:
```bash
cargo run -- --character fabelis.json
```

### Step 5: Run the Agent
```bash
cargo run
```
or with your custom character
```bash
cargo run -- --character fabelis.json
```

## Supported Integrations  (more to come...)

| Completion Providers | Embedding Providers | Databases | Clients |
|:-----------------:|:------------------:|:----------:|:--------:|
| Anthropic | Local | Local | API |
| Cohere | OpenAI | MongoDB | CLI |
| Gemini | XAI | - | Story-Telling |
| OpenAI | Local | - | Twitter |
| Perplexity | - | - | - |
| XAI | Cohere | - | - |

## Looking For More?
**View Our Docs [here](https://docs.fabelis.ai)**
 - **[EXAMPLES](https://docs.fabelis.ai/examples)**
 - **[SUPPORT](https://docs.fabelis.ai/support)**

---
<p align="center">Built with 🤖 and ❤️ by the Fabelis Team</p>