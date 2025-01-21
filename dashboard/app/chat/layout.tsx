"use client";

import { ChatProvider } from "@/providers/ChatProvider";

export default function Layout({ children }: { children: React.ReactNode }) {
  return <ChatProvider>{children}</ChatProvider>;
}
