import { useEffect, useRef } from 'react';

interface ReportBugDialogProps {
  comment: string;
  onCommentChange: (value: string) => void;
  onSubmit: () => Promise<void>;
  onClose: () => void;
  isSubmitting: boolean;
  error?: string | null;
}

export function ReportBugDialog({
  comment,
  onCommentChange,
  onSubmit,
  onClose,
  isSubmitting,
  error,
}: ReportBugDialogProps) {
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);

  useEffect(() => {
    textareaRef.current?.focus();
  }, []);

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/40" onClick={onClose} />

      <div className="relative bg-white rounded-2xl shadow-2xl w-[90%] max-w-md p-6 space-y-4">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">Report a Problem</h2>
          <p className="text-sm text-gray-600 mt-1">
            Share what happened. Recent app logs will be attached automatically.
          </p>
        </div>

        <div>
          <label htmlFor="bug-comment" className="sr-only">
            Bug report details
          </label>
          <textarea
            id="bug-comment"
            ref={textareaRef}
            value={comment}
            onChange={(event) => onCommentChange(event.target.value)}
            placeholder="Describe the issue you encountered..."
            className="w-full h-32 border border-gray-300 rounded-xl p-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            maxLength={2000}
            aria-describedby="bug-comment-help"
          />
          <div id="bug-comment-help" className="mt-1 text-xs text-gray-400 text-right">
            {comment.length}/2000
          </div>
        </div>

        {error && (
          <div className="text-sm text-red-600 bg-red-50 border border-red-200 rounded-lg px-3 py-2">
            {error}
          </div>
        )}

        <div className="flex items-center justify-end gap-3">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-800"
            disabled={isSubmitting}
          >
            Cancel
          </button>

          <button
            onClick={onSubmit}
            disabled={isSubmitting || comment.trim().length === 0}
            className="flex items-center justify-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg disabled:bg-blue-300 transition-colors"
            style={{ minWidth: 110 }}
          >
            {isSubmitting ? (
              <>
                <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                <span>Preparing...</span>
              </>
            ) : (
              <span>Send Report</span>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
