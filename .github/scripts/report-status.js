export async function reportStatus({github, context}) {
  const { owner, repo } = context.repo;

  await github.rest.repos.createCommitStatus({
    owner,
    repo,
    sha: context.sha,
    state: process.env.CONCLUSION,
    context: process.env.CHECK_NAME,
  });
}
