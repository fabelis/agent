"use client";
import { SettingsProvider } from "@/providers/SettingsProvider";

export default function Layout({ children }: { children: React.ReactNode }) {
  return <SettingsProvider>{children}</SettingsProvider>;
}
