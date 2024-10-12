use crate::{error::GetSubjectError, schemas::*};

use super::Client;

impl Client {
    pub async fn get_subject(&self, subject_id: SubjectId) -> Result<Subject, GetSubjectError> {
        let url = self
            .base_url
            .join(&format!("/v0/subjects/{}", subject_id))?;

        let response = self.get(url).send().await?.error_for_status()?;

        let subject = response.json().await?;

        Ok(subject)
    }

    pub async fn get_subject_persons(
        &self,
        subject_id: SubjectId,
    ) -> Result<Vec<RelatedPerson>, GetSubjectError> {
        let url = self
            .base_url
            .join(&format!("/v0/subjects/{}/persons", subject_id))?;

        let response = self.get(url).send().await?.error_for_status()?;

        let persons = response.json().await?;

        Ok(persons)
    }

    pub async fn get_subject_characters(
        &self,
        subject_id: SubjectId,
    ) -> Result<Vec<RelatedCharacter>, GetSubjectError> {
        let url = self
            .base_url
            .join(&format!("/v0/subjects/{}/characters", subject_id))?;

        let response = self.get(url).send().await?.error_for_status()?;

        let characters = response.json().await?;

        Ok(characters)
    }

    pub async fn get_subject_subjects(
        &self,
        subject_id: SubjectId,
    ) -> Result<Vec<SubjectRelation>, GetSubjectError> {
        let url = self
            .base_url
            .join(&format!("/v0/subjects/{}/subjects", subject_id))?;

        let response = self.get(url).send().await?.error_for_status()?;

        let subjects = response.json().await?;

        Ok(subjects)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn test_get_subject() -> anyhow::Result<()> {
        let client = Client::new(None);

        let subject = client.get_subject(3559).await?;

        assert_eq!(subject.name, "とある魔術の禁書目録");
        assert_eq!(subject.r#type, SubjectType::Book);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_subject_persons() -> anyhow::Result<()> {
        let client = Client::new(None);

        let persons = client.get_subject_persons(3559).await?;

        assert_eq!(persons.len(), 8);

        let person = persons.iter().find(|p| p.id == 3608);

        assert_eq!(person.map(|p| p.name.as_str()), Some("鎌池和馬"));
        assert_eq!(person.map(|p| p.relation.as_str()), Some("作者"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_subject_characters() -> anyhow::Result<()> {
        let client = Client::new(None);

        let characters = client.get_subject_characters(3559).await?;

        assert_eq!(characters.len(), 109);

        let character = characters.iter().find(|c| c.id == 3498);

        assert_eq!(character.map(|c| c.name.as_str()), Some("上条当麻"));
        assert_eq!(character.map(|c| c.relation.as_str()), Some("主角"));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_subject_subjects() -> anyhow::Result<()> {
        let client = Client::new(None);

        let subjects = client.get_subject_subjects(3559).await?;

        let subject = subjects.iter().find(|s| s.id == 3582);

        assert_eq!(
            subject.map(|s| s.name_cn.as_str()),
            Some("某科学的超电磁炮")
        );
        assert_eq!(subject.map(|s| s.relation.as_str()), Some("相同世界观"));

        Ok(())
    }
}
