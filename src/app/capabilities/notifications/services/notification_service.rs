use std::{collections::HashMap, path::Path};

use tera::{Context, Tera};

use crate::capabilities::{
    common::{config::config_service::ConfigService, mailer::Mailer},
    notifications::{
        models::notification_builder::{
            DefaultReplaceSettings, NotificationBuilder, NotificationConfig,
            NotificationConfigType, NotificationTriggerSettings,
        },
        templates::layout::add_layout,
    },
};

pub enum NotificationError {
    NotificationNotConfigured,
    MailerError(String),
}

#[derive(Clone)]
pub struct NotificationService {
    config: ConfigService,
    store: HashMap<String, NotificationConfig>,
    tera: Tera,
    mailer: Mailer,
}

impl NotificationService {
    pub fn new(config: ConfigService, mailer: Mailer) -> Self {
        let template_glob = &format!("{}/**/*.html", env!("CARGO_MANIFEST_DIR")).to_string();
        Self {
            config,
            store: HashMap::new(),
            tera: Tera::new(template_glob).unwrap(),
            mailer,
        }
    }

    /// get notification builder
    pub fn get_builder(&self, id: String, ntype: NotificationConfigType) -> NotificationBuilder {
        return NotificationBuilder::new(id, ntype);
    }

    /// register notification
    pub fn register(&mut self, setting: NotificationConfig) {
        let id = setting.id.clone();
        self.store.insert(id.clone(), setting);
        tracing::info!("Registered Notification: {}", id);
    }

    pub async fn trigger(
        &mut self,
        id: String,
        trigger_settings: NotificationTriggerSettings,
    ) -> Result<bool, NotificationError> {
        let settings = self.store.get(&id);
        match settings {
            None => {
                return Err(NotificationError::NotificationNotConfigured);
            }
            Some(setting) => match setting.ntype {
                NotificationConfigType::Email => {
                    return self.handle_email(setting.clone(), trigger_settings).await
                }
                NotificationConfigType::Websocket => {
                    return self
                        .handle_websocket(setting.clone(), trigger_settings)
                        .await
                }
            },
        }
    }

    pub async fn handle_email(
        &mut self,
        conf: NotificationConfig,
        trigger_settings: NotificationTriggerSettings,
    ) -> Result<bool, NotificationError> {
        let params = self.get_merged_values(conf.settings, trigger_settings.clone());
        let html_content = self.get_text(conf.render, params.clone());
        let body = add_layout(html_content);
        let subject = self.get_text(conf.subject.unwrap(), params.clone());
        match self
            .mailer
            .send_email(subject, body, trigger_settings.email.unwrap(), None)
            .await
        {
            crate::capabilities::common::mailer::SendEmailResult::Ok(_) => Ok(true),
            crate::capabilities::common::mailer::SendEmailResult::Error(e) => {
                Err(NotificationError::MailerError(e))
            }
        }
    }

    pub async fn handle_websocket(
        &self,
        conf: NotificationConfig,
        trigger_settings: NotificationTriggerSettings,
    ) -> Result<bool, NotificationError> {
        Err(NotificationError::NotificationNotConfigured)
    }

    fn get_merged_values(
        &self,
        default_params: HashMap<String, DefaultReplaceSettings>,
        trigger_settings: NotificationTriggerSettings,
    ) -> HashMap<String, String> {
        let mut merged = HashMap::<String, String>::new();

        for (key, val) in default_params.iter() {
            let mut new_val = trigger_settings.params.get(key);
            if new_val.is_none() {
                if val.default {
                    new_val = Some(&val.example_value);
                }
            }
            merged.insert(key.to_string(), new_val.unwrap_or(&"".to_string()).to_string());
        }
        merged
    }

    fn get_text(&mut self, template: String, params: HashMap<String, String>) -> String {
        self.tera.add_template_file(template.clone(), None).unwrap();
        let mut context = Context::new();
        for (key, val) in params.iter() {
            context.insert(key, val);
        }
        self.tera.render(&template, &context).unwrap()
    }
}
