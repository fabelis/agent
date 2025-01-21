import Link from "next/link";

export default function NotFound() {
  return (
    <div className="w-full h-full flex flex-col items-center justify-center tracking-tight">
      <h1 className="text-9xl font-bold">404</h1>
      <p className="text-2xl font-semibold text-muted-foreground">
        Page Not Found
      </p>
      <Link href="/" className="text-sm mt-2 hover:underline">
        Return to Home
      </Link>
    </div>
  );
}
