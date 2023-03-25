use std::sync::Arc;

use futures::lock::Mutex;
use uuid::Uuid;

use crate::market_prompt::market_prompt_manager::MarketPromptManager;
use crate::market_prompt::market_prompt_repo::PromptMarketRepo;
use crate::market_prompt::{MarketPrompt, MarketPromptIndex};
use crate::prompt::prompt_manager::PromptManager;
use crate::result::Result;
use crate::setting::Setting;

pub struct PromptMarketService {
    prompt_market_manager: Arc<Mutex<MarketPromptManager>>,
    prompt_manager: Arc<Mutex<PromptManager>>,
    setting: Arc<Mutex<Setting>>,
}

impl PromptMarketService {
    pub fn new(
        prompt_market_manager: Arc<Mutex<MarketPromptManager>>,
        prompt_manager: Arc<Mutex<PromptManager>>,
        setting: Arc<Mutex<Setting>>,
    ) -> Self {
        Self {
            prompt_market_manager,
            prompt_manager,
            setting,
        }
    }

    pub async fn repos(&self) -> Result<Vec<PromptMarketRepo>> {
        let client = self.setting.lock().await.create_client()?;
        let repos = PromptMarketRepo::list(client).await?;
        let repos_cloned = repos.iter().map(|r| r.partial_clone()).collect();

        self.prompt_market_manager
            .lock()
            .await
            .set_repos(repos_cloned)
            .await?;

        Ok(repos)
    }

    pub async fn index_list(&self, name: &str) -> Result<Vec<MarketPromptIndex>> {
        let client = self.setting.lock().await.create_client()?;
        let index_list = self
            .prompt_market_manager
            .lock()
            .await
            .index_list(name, client)
            .await?;

        Ok(index_list)
    }

    pub async fn load(&self, name: &str, id: Uuid) -> Result<MarketPrompt> {
        let client = self.setting.lock().await.create_client()?;
        let prompt = self
            .prompt_market_manager
            .lock()
            .await
            .load_data(name, id, client)
            .await?;

        Ok(prompt)
    }

    pub async fn install(&mut self, prompt: &MarketPrompt) -> Result<()> {
        self.prompt_manager
            .lock()
            .await
            .create(&prompt.act, &prompt.prompt)
            .await?;

        Ok(())
    }
}
