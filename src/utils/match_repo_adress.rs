use crate::GHProfile;

#[derive(Debug, PartialEq)]
pub enum RepoAdressType {
    OwnedByCurrentUser,        // clone
    SpecifiedCurrentUsersRepo, // clone my-repo
    OwnedByStrangeUser,        // clone github-nickname
    SpecifiedUserAndRepo,      // clone github-nickname/his-repo
}

pub fn match_repo_adress(
    current_user: impl Into<String>,
    argument: Option<&str>,
) -> RepoAdressType {
    let current_user = current_user.into();

    use RepoAdressType::*;
    let argument: String = match argument {
        Some(argument) => argument.into(),
        None => return OwnedByCurrentUser,
    };

    if argument.find('/').is_some() {
        return SpecifiedUserAndRepo;
    }

    let current_user = GHProfile::new(current_user);

    if current_user.repo_exists(&argument) {
        return SpecifiedCurrentUsersRepo;
    }

    OwnedByStrangeUser
}

#[cfg(test)]
mod test_match_repo_adress {
    use super::{RepoAdressType::*, *};

    #[test]
    fn returns_none_when_argument_is_none() {
        assert_eq!(match_repo_adress("Milesq", None), OwnedByCurrentUser);
    }

    #[test]
    fn returns_specified_user_and_repo() {
        assert_eq!(
            match_repo_adress("Milesq", Some("Milesq/awesome-project")),
            SpecifiedUserAndRepo
        );
    }
    #[test]
    fn returns_specified_current_users_repo() {
        assert_eq!(
            match_repo_adress("Milesq", Some("clone-github-repo")),
            SpecifiedCurrentUsersRepo
        );
    }
    #[test]
    fn returns_owned_by_strange_user() {
        assert_eq!(
            match_repo_adress("Milesq", Some("loremipsumasdasdhabfaksdjhfjahsahsdkasdjh")),
            OwnedByStrangeUser
        );
    }
}
