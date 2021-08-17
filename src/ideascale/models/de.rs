use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, Clone)]
pub struct AdaRewards(#[serde(deserialize_with = "deserialize_rewards")] u64);

#[derive(Debug, Deserialize, Clone)]
pub struct Challenge {
    pub id: u32,
    #[serde(alias = "name", deserialize_with = "deserialize_clean_challenge_title")]
    pub title: String,
    #[serde(alias = "tagline")]
    pub rewards: AdaRewards,
    pub description: CleanString,
    #[serde(alias = "groupId")]
    pub fund_id: u32,
    #[serde(alias = "funnelId")]
    pub funnel_id: u32,
    #[serde(alias = "campaignUrl")]
    pub challenge_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Funnel {
    pub id: u32,
    #[serde(alias = "name")]
    pub title: CleanString,
    pub description: CleanString,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fund {
    pub id: u32,
    pub name: CleanString,
    #[serde(alias = "campaigns")]
    pub challenges: Vec<Challenge>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proposal {
    #[serde(alias = "id")]
    pub proposal_id: u32,
    pub proposal_category: Option<CleanString>,
    #[serde(alias = "title")]
    pub proposal_title: CleanString,
    #[serde(alias = "text")]
    pub proposal_summary: CleanString,

    #[serde(alias = "url")]
    pub proposal_url: String,
    #[serde(default)]
    pub proposal_files_url: String,

    #[serde(alias = "customFieldsByKey")]
    pub custom_fields: Option<ProposalCustomFieldsByKey>,

    #[serde(alias = "authorInfo")]
    pub proposer: Proposer,

    #[serde(alias = "stageId")]
    pub stage_id: u32,

    #[serde(alias = "stageLabel")]
    pub stage_type: String,

    #[serde(alias = "campaignId")]
    pub challenge_id: u32,

    #[serde(alias = "flag", deserialize_with = "deserialize_approved")]
    pub approved: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proposer {
    pub name: String,
    #[serde(alias = "email")]
    pub contact: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProposalCustomFieldsByKey {
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stage {
    #[serde(default)]
    pub label: String,
    #[serde(alias = "funnelId", default)]
    pub funnel_id: u32,
    #[serde(alias = "assessmentId", default)]
    pub assessment_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CleanString(#[serde(deserialize_with = "deserialize_clean_string")] String);

impl Funnel {
    pub fn is_community(&self) -> bool {
        self.title.as_ref().contains("Challenge Setting")
    }
}

impl ToString for CleanString {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl AsRef<str> for CleanString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<AdaRewards> for u64 {
    fn from(rewards: AdaRewards) -> Self {
        rewards.0
    }
}

impl Display for AdaRewards {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn deserialize_approved<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let approved = String::deserialize(deserializer)?;
    Ok(matches!(approved.as_str(), "approved"))
}

pub fn clean_str(s: &str) -> String {
    let mut result = s.to_string();
    result.retain(|c| !matches!(c, '*' | '-' | '/'));
    result
}

fn deserialize_clean_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let rewards_str = String::deserialize(deserializer)?;
    Ok(clean_str(&rewards_str))
}

fn deserialize_clean_challenge_title<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let mut rewards_str = String::deserialize(deserializer)?;
    // Remove leading `FX: `
    if rewards_str.starts_with('F') {
        if let Some(first_space) = rewards_str.find(' ') {
            let (_, content) = rewards_str.split_at(first_space + 1);
            rewards_str = content.to_string();
        }
    }
    Ok(rewards_str)
}

fn deserialize_rewards<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let rewards_str = String::deserialize(deserializer)?;

    // input is not standarized, hack an early return if it is just 0 ada
    if rewards_str.starts_with("0 ada") {
        return Ok(0);
    }
    sscanf::scanf!(rewards_str.trim_end(), "${} in ada", String)
        // trim all . or , in between numbers
        .map(|mut s: String| {
            s.retain(|c: char| c.is_numeric() || matches!(c, '.'));
            s
        })
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            D::Error::custom(&format!("Unable to read malformed value: {}", rewards_str))
        })
}

#[cfg(test)]
mod test {
    use crate::ideascale::models::de::Proposal;

    #[test]
    fn deserialize_proposal() {
        let fake_data = r#"{
        "id": 352809,
        "creationDateTime": "2021-04-14T02:34:46.000-07:00",
        "editedAt": "2021-08-11T19:16:02.000-07:00",
        "statusChangeDate": "2021-08-11T19:16:02.000-07:00",
        "title": "IdeaScale Improvements",
        "text": "Obstacles prevent Fund Proposers from expressing themselves in the Cardano community and Community Advisors from reviewing proposals.",
        "campaignId": 25942,
        "campaignName": "F5: Distributed decision making",
        "authorId": 3084011,
        "authorInfo": {
            "id": 3084011,
            "name": "Erik Siebert",
            "personId": 830578,
            "email": "erik.siebert@ideascale.com",
            "emailHash": "84a723b017f031065f92dd9d03511009",
            "userName": "eriksiebert",
            "globalModerator": false,
            "admin": false,
            "customRole": false,
            "registeredDateTime": "2021-04-14T09:08:39.000-07:00",
            "modifiedAt": "2021-08-10T03:54:18.000-07:00",
            "lastAccess": "2021-04-27T13:11:16.000-07:00",
            "source": "web",
            "status": "member/Verified",
            "ideaCount": 1,
            "voteCount": 0,
            "commentCount": 7,
            "tosAccepted": false,
            "avatarUrl": "https://secure.gravatar.com/avatar/5327150d818886b1211e375f631e9551.jpg?s=80&d=https%3A%2F%2Fstatic.ideascale.com%2Fimages%2Favatar%2Fdefault-E.png",
        "points": 157,
        "profileQuestions": {
            "For Community advisors only: What is your ada payment address? (Must be a Shelly address, starting with \"addr\")": "",
            "Want to register as a community advisor? Confirm all following statements are true: I want to serve as a community advisor. I did not submit a funding proposal for Fund4. I am not affiliated with any proposing team in Fund4. I commit to provide fair and thoughtful reviews.": "",
            "Want to register as a community advisor? Confirm all following statements are true: I want to serve as a community advisor. I did not submit a funding proposal for Fund3. I am not affiliated with any proposing team in Fund3. I commit to provide fair and thoughtful reviews.": "",
            "Want to register as a community advisor? Confirm all following statements are true: I want to serve as a community advisor. I did not submit a proposal for Fund2. I am not affiliated with any proposing team in Fund2. I commit to provide fair and thoughtful reviews": "",
            "Which of these definitions apply to you?": "Other/Prefer not to say",
            "Want to register as a community advisor? Confirm all following statements are true: I want to serve as a community advisor. I did not submit a funding proposal for Fund5. I am not affiliated with any proposing team in Fund5. I commit to provide fair and thoughtful reviews.": ""
        },
        "kudoCount": 18
    },
    "voteCount": 0,
    "upVoteCount": 0,
    "downVoteCount": 0,
    "myVote": 0,
    "commentCount": 25,
    "url": "https://cardano.ideascale.com/a/dtd/352809-48088",
    "tags": [],
    "funnelId": 7138,
    "funnelName": "Fund 5 Funnel",
    "statusId": 78487,
    "status": "stage-governancephasef3b0b2",
    "stageId": 78487,
    "stageName": "stage-governancephasef3b0b2",
    "stageLabel": "Governance phase",
    "flag": "approved",
    "customFieldsByKey": {
    "email_of_person_who_referred_you_to_become_a_proposer": "",
    "problem_solution": "We are proposing making improvements to the IdeaScale platform will allow the Cardano community to make better decisions.",
    "how_did_referral_refer_you_to_submit_a_proposal___please_elaborate_on_details_as_much_as_possible_": "",
    "detailed_plan__not_required__-_fill_in_here_any_additional_details": "IdeaScale would like to address a few concerns that the community has expressed.\n\n  \n**Proposal Drafting**  \nTo aid in the drafting of proposals we will add additional **markdown syntax support** as well as WYSIWYG style editor. We've heard from multiple users their frustration with formatting proposals. We hope adding both markdown as well as an editor will ease those concerns.\n\n  \n**Browsing Proposals**  \nIdeaScale proposes adding a **Compact View** that will allow both voters and community advisors to browse a condensed list of submitted proposals. Compact View will include only essential elements such as Title, Campaign Name, Submitter, and Date. This will allow members to view more proposals per page. Additionally IdeaScale will mark all proposals that have been edited by the author with an **Edited Label**.\n\n  \n**Community Advisors**  \nTo support the work of the Community Advisors, IdeaScale proposes the following changes:\n\n*   Adding a **Key**, to the Assessment Stage so that the community can align on what a 5-Star Review means vs. a 1-Star. Key will be persistent when CA's assess proposals.\n*   Add an Administrator option that will allow **Required Review Notes** for each review.\n*   Add an Administrator option that will allow **Anonymized** **Review Notes** to be displayed to the community.\n*   Add a better means of displaying what proposals CA have already reviewed.",
    "requested_funds_old": "10000",
    "self-assessment_checklist_regular": "",
    "relevant_experience": "Over the last 11 years IdeaScale has served over 10,000 communities from large enterprises to SMBs.",
    "website_github_repository__not_required_": "",
    "ada_payment_address": "addr1q8v8ljkkhcatpvypyckd2q8q73lfv6rxda52adhrh6j700gqpeychw0x044g2d65af5gn8r7e4sjzj7acegem390yr6swzf7c7"
    },
    "campaignCustomFields": {
    "Relevant experience": "Over the last 11 years IdeaScale has served over 10,000 communities from large enterprises to SMBs.",
    "Requested funds in USD_old": "10000",
    "Proposer ada payment address (Must be a Shelly address, starting with \"addr\")": "addr1q8v8ljkkhcatpvypyckd2q8q73lfv6rxda52adhrh6j700gqpeychw0x044g2d65af5gn8r7e4sjzj7acegem390yr6swzf7c7",
    "Website/GitHub repository (not required)": "",
    "Email of person who referred you to become a proposer": "",
    "How did the referral refer you to submit a proposal? (Please elaborate on details as much as possible)": "",
    "Describe your solution to the problem": "We are proposing making improvements to the IdeaScale platform will allow the Cardano community to make better decisions.",
    "Self-Assessment checklist": "",
    "Detailed plan (not required) - Fill in here any additional details": "IdeaScale would like to address a few concerns that the community has expressed.\n\n  \n**Proposal Drafting**  \nTo aid in the drafting of proposals we will add additional **markdown syntax support** as well as WYSIWYG style editor. We've heard from multiple users their frustration with formatting proposals. We hope adding both markdown as well as an editor will ease those concerns.\n\n  \n**Browsing Proposals**  \nIdeaScale proposes adding a **Compact View** that will allow both voters and community advisors to browse a condensed list of submitted proposals. Compact View will include only essential elements such as Title, Campaign Name, Submitter, and Date. This will allow members to view more proposals per page. Additionally IdeaScale will mark all proposals that have been edited by the author with an **Edited Label**.\n\n  \n**Community Advisors**  \nTo support the work of the Community Advisors, IdeaScale proposes the following changes:\n\n*   Adding a **Key**, to the Assessment Stage so that the community can align on what a 5-Star Review means vs. a 1-Star. Key will be persistent when CA's assess proposals.\n*   Add an Administrator option that will allow **Required Review Notes** for each review.\n*   Add an Administrator option that will allow **Anonymized** **Review Notes** to be displayed to the community.\n*   Add a better means of displaying what proposals CA have already reviewed."
    },
"ideaNumber": 6787,
"locationInfo": {
"city": "CA",
"country": "US",
"states": "San Leandro",
"areaCode": 0,
"ip": "73.189.144.173",
"source": "gps",
"latitude": 37.7363899,
"longitude": -122.1380466,
"regionName": "California",
"accuracy": -1.0,
"altitude": 0.0,
"altitudeAccuracy": -1.0,
"heading": -1.0,
"speed": -1.0
},
"ideaPermissionInfo": {
"canVote": false,
"canRetractVote": false,
"canComment": true
},
    "labels": []
}"#;

        let _proposal: Proposal = serde_json::from_str(&fake_data).unwrap();
    }
}
