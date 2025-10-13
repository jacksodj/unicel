import { ReactNode } from 'react';

interface TooltipProps {
  content: string;
  children: ReactNode;
  variant?: 'default' | 'warning' | 'error';
}

export default function Tooltip({ content, children, variant = 'default' }: TooltipProps) {
  const variantStyles = {
    default: 'bg-gray-900 text-white',
    warning: 'bg-orange-600 text-white',
    error: 'bg-red-600 text-white',
  };

  return (
    <div className="group relative inline-block">
      {children}
      <div
        className={`
          absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1
          text-xs rounded shadow-lg whitespace-nowrap
          opacity-0 invisible group-hover:opacity-100 group-hover:visible
          transition-opacity duration-200 pointer-events-none z-50
          ${variantStyles[variant]}
        `}
      >
        {content}
        {/* Arrow */}
        <div
          className={`
            absolute top-full left-1/2 -translate-x-1/2 -mt-1
            border-4 border-transparent
            ${variant === 'warning' ? 'border-t-orange-600' : ''}
            ${variant === 'error' ? 'border-t-red-600' : ''}
            ${variant === 'default' ? 'border-t-gray-900' : ''}
          `}
        />
      </div>
    </div>
  );
}
