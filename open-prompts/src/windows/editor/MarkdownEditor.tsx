interface Props {
  content: string;
  onChange: (content: string) => void;
}

export function MarkdownEditor({ content, onChange }: Props) {
  return (
    <textarea
      className="markdown-editor"
      value={content}
      onChange={(event) => onChange(event.target.value)}
      placeholder="Write your prompt here..."
      spellCheck={false}
    />
  );
}
