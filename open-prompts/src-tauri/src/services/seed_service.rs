use crate::error::AppResult;
use crate::models::prompt::{Prompt, PromptIndex, PromptMetadata};
use crate::services::index_service::save_index;
use crate::services::prompt_service::save_prompt;
use crate::services::storage::StoragePaths;

pub fn seed_if_needed(paths: &StoragePaths, index: &mut PromptIndex) -> AppResult<()> {
    if index.seeded {
        return Ok(());
    }

    if !index.prompts.is_empty() {
        index.seeded = true;
        save_index(paths, index)?;
        return Ok(());
    }

    // Seed "Summarize" prompt
    let summarize_prompt = Prompt {
        meta: PromptMetadata {
            id: String::new(), // save_prompt will generate UUID
            name: "Summarize".to_string(),
            folder: "Writing".to_string(),
            description: "Summarize the content".to_string(),
            filename: String::new(),
            use_count: 0,
            last_used: None,
            created: String::new(),
            updated: String::new(),
            icon: Some("file-text".to_string()),
            color: None,
        },
          content: r#"
# **Task**

Transform the provided content into a structured summary. Extract and organize key information while maintaining the document's original terminology and structure.

# **Objectives**

- Preserve technical accuracy and relationships
- Maintain context without external interpretation

# **Instructions**

1. **Main Ideas (typically 3-8 points)**:
    - List the primary concepts or arguments
    - Include fewer points if the source is brief; omit section if no main ideas exist
    - Provide brief summaries

2. **Key Points (typically 3-8 points)**:
    - Identify important supporting details or secondary themes
    - Relate each point to a main idea
    - Use language similar to the original text

3. **Quotes (3-8 most relevant)**:
    - Select exact passages that:
      * Define key concepts or processes
      * Contain critical instructions or warnings
      * Represent unique insights not captured elsewhere
    - Include context indicators if quote references something specific

4. **Stated Facts (typically 3-8 points)**:
    - List explicit factual claims made in the text
    - Present the information as it appears
    - For process flows, maintain the sequence

5. **Document Metadata** (if present):
    - Document type/category
    - System names or tools mentioned
    - Key personnel roles (not names) referenced
    - Related processes or workflows

6. **Relationships** (if applicable):
    - Systems or processes that interact
    - Prerequisites or dependencies mentioned
    - Cause-and-effect relationships stated in the text

# **Instructions for Technical Content**

- Preserve technical terms, system names, and file paths exactly as written
- Include configuration details, error codes, and process steps in "Stated Facts"
- Maintain hierarchical relationships (e.g., numbered steps, sub-processes)
- Exclude code snippets but preserve:
  * File paths and system locations
  * Configuration values and parameters
  * Command names and error messages
  * API endpoints and service names

# **Handling Structured Content**

- Preserve numbered lists and hierarchical information in "Stated Facts"
- Maintain the relationship between parent and child items
- For process flows, maintain the sequence in your summary

# **Output Guidelines**

- **Include sections**: "Main Ideas," "Key Points," "Quotes," "Stated Facts," and optionally "Document Metadata" and "Relationships"
- Use numbered or bulleted lists for all sections
- Clearly label each section
- Use language and terminology consistent with the original text
- Note any unclear or contradictory text without attempting to resolve it
- Do not include information not explicitly stated in the text
- Maintain a neutral tone focused on reporting content
- Use neutral language that doesn't reveal personal information through context
- Omit any section if no relevant content exists for it

# **Final Check**

Before submitting your summary, ensure:

1. Neutral language is used throughout, avoiding inadvertent revelations of personal information
2. Only information from the original text is included—no external knowledge or context
3. Language and concepts are consistent with the provided content
4. The summary accurately reflects the text without additional interpretation
5. Irrelevant sections with no content are omitted
6. Technical terms and system references are preserved exactly as written (except PII)
7. Relationships and dependencies are clearly documented if present

# **Example Output**

**Main Ideas:**

1. **Idea 1**: Brief summary of the first main idea
2. **Idea 2**: Brief summary of the second main idea

**Key Points:**

1. **Key Point 1**: Supporting detail related to Idea 1
2. **Key Point 2**: Supporting detail related to Idea 2

**Quotes:**

1. "[Exact passage representing a central theme]" 
2. "[Another significant passage]"

**Stated Facts:**

1. **Fact 1**: Explicit factual claim as it appears in the text
2. **Fact 2**: Another factual claim (contact: [Email Address])
3. **Process Step**: Step-by-step process maintaining original sequence

**Document Metadata:**

- **Type**: Technical documentation / Process guide
- **Systems**: System A, System B, API Name
- **Key Contacts**: [Role 1], [Role 2]

**Relationships:**

1. System A sends requests to System B
2. Process X must complete before Process Y begins
3. [Department Head] approves all changes to Process Z

**Note**: This example demonstrates the expected format of the output. Sections should be omitted if no relevant content exists."#
                .to_string(),
    };
    save_prompt(paths, index, summarize_prompt)?;

    // Seed "Improve Writing" prompt
    let improve_prompt = Prompt {
        meta: PromptMetadata {
            id: String::new(),
            name: "Markov Chain State".to_string(),
            folder: "AnalyzeCode".to_string(),
            description: "Find all scary bugs".to_string(),
            filename: String::new(),
            use_count: 0,
            last_used: None,
            created: String::new(),
            updated: String::new(),
            icon: Some("pencil".to_string()),
            color: None,
        },
        content: "Create a full Markov Chain state graph to find any possible flaws in this"
            .to_string(),
    };
    save_prompt(paths, index, improve_prompt)?;

    // Seed "Critical Thinking" prompt
    let critical_thinking_prompt = Prompt {
        meta: PromptMetadata {
            id: String::new(),
            name: "Critical Thinking".to_string(),
            folder: "AnalyzeCode".to_string(),
            description: "Generate alternatives and perspectives".to_string(),
            filename: String::new(),
            use_count: 0,
            last_used: None,
            created: String::new(),
            updated: String::new(),
            icon: Some("lightbulb".to_string()),
            color: None,
        },
        content: r#"
# Critical Thinking

Think of 3 alternative ways this could have been solved:

    - [Alternative approach 1]
    - [Alternative approach 2]
    - [Alternative approach 3]

Come up with 3 different ideas or perspectives:

    - [Idea or perspective 1]
    - [Idea or perspective 2]
    - [Idea or perspective 3]"#
            .to_string(),
    };
    save_prompt(paths, index, critical_thinking_prompt)?;

    index.seeded = true;
    save_index(paths, index)?;

    Ok(())
}
