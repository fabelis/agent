import * as React from "react";
import { cn } from "@/lib/utils";

interface ProgressSpinnerProps extends React.SVGAttributes<SVGSVGElement> {
  size?: number;
  strokeWidth?: number;
}

const ProgressSpinner = React.forwardRef<SVGSVGElement, ProgressSpinnerProps>(
  ({ className, size = 24, strokeWidth = 2, ...props }, ref) => {
    return (
      <svg
        className={cn("animate-spin text-muted-foreground", className)}
        xmlns="http://www.w3.org/2000/svg"
        width={size}
        height={size}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth={strokeWidth}
        strokeLinecap="round"
        strokeLinejoin="round"
        {...props}
        ref={ref}
      >
        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
      </svg>
    );
  }
);

ProgressSpinner.displayName = "ProgressSpinner";

export { ProgressSpinner };
