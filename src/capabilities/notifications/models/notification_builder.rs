use std::collections::HashMap;

use super::super::super::common::mailer::MailUser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NotificationConfigType {
    Email,
    Websocket
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UserNotificationType {
    LoginLink,
    VerificationLink
}

#[derive(Debug, Clone)]
pub struct DefaultReplaceSettings {
    pub example_value: String,
    pub default: bool,
}

#[derive(Clone)]
pub struct NotificationTriggerSettings {
    pub email: Option<MailUser>,
    pub user_id: i32,
    pub params: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NotificationBuilderError {
    SubjectHandlerNotSet,
    RenderHandlerNotSet,
    NotificationTypeNotSet
}

#[derive(Clone)]
pub struct NotificationConfig {
    pub id: String,
    pub ntype: NotificationConfigType,
    pub untype: UserNotificationType,
    pub subject: Option<String>,
    pub render: String,
    pub settings: HashMap<String, DefaultReplaceSettings>
}

#[derive(Clone)]
pub struct NotificationBuilder {
    id: String,
    ntype: NotificationConfigType,
    subject_template: Option<String>,
    render_template: Option<String>,
    untype: Option<UserNotificationType>,
    replace_settings: HashMap<String, DefaultReplaceSettings>
}

impl NotificationBuilder {
    pub fn new(id: String, ntype: NotificationConfigType) -> Self {
        Self {
            id,
            ntype,
            subject_template: None,
            render_template: None,
            untype: None,
            replace_settings: HashMap::new()
        }
    }
    pub fn set_subject_template(&mut self, template: String) -> &Self {
        self.subject_template = Some(template);
        self
    }

    pub fn set_render_template(&mut self, template: String) -> &Self {
        self.render_template = Some(template);
        self
    }

    pub fn set_notification_type(&mut self, untype: UserNotificationType) -> &Self {
        self.untype = Some(untype);
        self
    }

    pub fn add_replace_settings(&mut self, key: String, example_value: String, default: bool) -> &Self {
        self.replace_settings.insert(key, DefaultReplaceSettings {
            example_value,
            default
        });
        self
    }

    pub fn build(&self) -> Result<NotificationConfig, NotificationBuilderError> {
        if self.ntype == NotificationConfigType::Email && self.subject_template == None {
            return Err(NotificationBuilderError::SubjectHandlerNotSet);
        }

        if self.render_template.is_none() {
            return Err(NotificationBuilderError::RenderHandlerNotSet);
        }

        if self.untype.is_none() {
            return Err(NotificationBuilderError::NotificationTypeNotSet);
        }

        return Ok(NotificationConfig {
            id: self.id.clone(),
            ntype: self.ntype.clone(),
            render: self.render_template.clone().unwrap().clone(),
            settings: self.replace_settings.clone(),
            subject: self.subject_template.clone(),
            untype: self.untype.unwrap().clone()
        })
    }
}