use crate::id_provider::{IDProvider, NanoIDProvider};

pub struct CreateShortUrl<I>
where
    I: IDProvider,
{
    id_provider: I,
}

impl<I> CreateShortUrl<I>
where
    I: IDProvider,
{
    pub fn new(id_provider: I) -> Self {
        Self { id_provider }
    }
    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        //Given
        let idp = NanoIDProvider;
        let command = CreateShortUrl::new(idp);
        //When
        let result = command.execute("https://www.google.com".to_owned()).await;
        //Then
        assert_ne!(result, Ok("".to_owned()));
    }

    #[tokio::test]
    async fn get_two_different_short_url() {
        //Given
        let idp = NanoIDProvider;
        let command = CreateShortUrl::new(idp);
        //When
        let result = command.execute("https://www.google.com".to_owned()).await;
        let result2 = command.execute("https://www.google.com".to_owned()).await;
        //Then
        assert_ne!(result, result2);
    }
}
