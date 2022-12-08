use std::ops::{Deref, DerefMut};

use crate::{
    error::{Error, Result},
    states::{Down, State, Up},
};

const GATEWAY_URL: &str = "http://detectportal.firefox.com/canonical.html";

pub struct Account<S: State> {
    username: String,
    password: String,
    state: S,
}

impl Account<Down> {
    pub fn new<S>(username: S, password: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            username: username.as_ref().into(),
            password: password.as_ref().into(),
            state: Down,
        }
    }
    pub async fn login(self) -> Result<Account<Up>> {
        let client = reqwest::Client::new();

        let body = client
            .get(GATEWAY_URL)
            .send()
            .await
            .map_err(Error::ReqwestError)?
            .bytes()
            .await
            .map(|i| String::from_utf8_lossy(&i).into_owned())
            .map_err(Error::ReqwestError)?;

        if body.starts_with("<meta") || body.len() < 112 {
            return Err(Error::PortalNotAvailable);
        }

        let token = &body[95..111];
        let redirect_url = &body[59..111];

        client
            .get(redirect_url)
            .send()
            .await
            .map_err(Error::ReqwestError)?;

        let login_data = format!(
            "4Tredir=http%3A%2F%2Fdetectportal.firefox.com%2Fcanonical.html\
            &magic={token}\
            &username={}\
            &password={}",
            self.username, self.password
        );

        client
            .post("https://access.um.ac.ir/")
            .body(login_data)
            .send()
            .await
            .map_err(Error::ReqwestError)?;

        Ok(Account {
            username: self.username,
            password: self.password,
            state: Up,
        })
    }

    pub async fn logout() -> Result<()> {
        logout().await
    }
}

impl Account<Up> {
    pub async fn logout(self) -> Result<Account<Down>> {
        logout().await?;

        Ok(Account {
            username: self.username,
            password: self.password,
            state: Down,
        })
    }
}

impl<S: State> Deref for Account<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<S: State> DerefMut for Account<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

pub async fn logout() -> Result<()> {
    reqwest::get("https://access.um.ac.ir/logout?")
        .await
        .map_err(Error::ReqwestError)
        .map(|_| ())
}
