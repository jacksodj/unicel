#!/usr/bin/env bash
# Set a single-select organization issue field on an issue in this repo, by
# name: Horizon (Now / Next / Later) or Priority (Urgent / High / Medium / Low).
# Dates are kept by the roadmap-dates workflow; don't set them by hand.
#
#   scripts/issue-field.sh <issue-number> <field> <option>
#   scripts/issue-field.sh 12 Horizon Now
set -euo pipefail

number="${1:?usage: issue-field.sh <issue-number> <field> <option>}"
field="${2:?usage: issue-field.sh <issue-number> <field> <option>}"
option="${3:?usage: issue-field.sh <issue-number> <field> <option>}"
repo="$(gh repo view --json nameWithOwner -q .nameWithOwner)"
owner="${repo%%/*}"; name="${repo#*/}"

# shellcheck disable=SC2016  # $o, $n, $i are GraphQL variables, not shell ones
ids="$(gh api graphql -f query='query($o:String!,$n:String!,$i:Int!){repository(owner:$o,name:$n){issue(number:$i){id} issueFields(first:50){nodes{... on IssueFieldSingleSelect{id name options{id name}}}}}}' \
  -F o="$owner" -F n="$name" -F i="$number" \
  --jq "[.data.repository.issue.id, (.data.repository.issueFields.nodes[] | select(.name == \"$field\") | .id), (.data.repository.issueFields.nodes[] | select(.name == \"$field\") | .options[] | select(.name == \"$option\") | .id)] | @tsv")"
read -r issue_id field_id option_id <<<"$ids"
[ -n "${option_id:-}" ] || { echo "issue-field: no field '$field' with option '$option' in $repo" >&2; exit 1; }

# shellcheck disable=SC2016  # GraphQL variables
gh api graphql -f query='mutation($i:ID!,$f:ID!,$o:ID!){setIssueFieldValue(input:{issueId:$i,issueFields:[{fieldId:$f,singleSelectOptionId:$o}]}){clientMutationId}}' \
  -F i="$issue_id" -F f="$field_id" -F o="$option_id" --silent
echo "issue-field: #$number $field = $option"
