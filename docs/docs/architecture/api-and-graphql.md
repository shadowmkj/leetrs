---
id: api-and-graphql
title: API & GraphQL Integration
sidebar_position: 2
---

# 🌐 API & GraphQL Integration

`leetrs` communicates directly with LeetCode's backend endpoints using `reqwest` over HTTPS.

---

## 🔑 Authentication & Headers

Every request issued by `LeetCodeClient` attaches:
1. `Cookie: LEETCODE_SESSION=<session>; csrftoken=<token>`
2. `x-csrftoken: <token>`
3. `Referer: https://leetcode.com`
4. `User-Agent: Mozilla/5.0 ...`

---

## 📡 GraphQL Endpoint

`leetrs` uses LeetCode's central GraphQL endpoint (`https://leetcode.com/graphql`) for:
- **`problemsetQuestionList`**: Fetching problem list, difficulty, solved status, and topic tags.
- **`questionData`**: Fetching detailed problem description HTML and language-specific code snippets.
- **`userStatus`**: Fetching user profile info (username, premium status).

### GraphQL Payload Structure

```json
{
  "query": "query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title titleSlug content codeSnippets { lang langSlug code } } }",
  "variables": {
    "titleSlug": "two-sum"
  },
  "operationName": "questionData"
}
```

---

## ⚡ Judge REST Endpoints

Code execution and submission utilize LeetCode's REST API:
- **`POST https://leetcode.com/problems/<slug>/interpret_solution/`**: Submits code for testing sample test cases.
- **`POST https://leetcode.com/problems/<slug>/submit/`**: Submits code for official judging.
- **`GET https://leetcode.com/submissions/detail/<interpret_id>/check/`**: Asynchronously polls execution status.
