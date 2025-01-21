import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { Separator } from "@/components/ui/separator";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { Sidebar } from "@/components/Sidebar";
import "./globals.css";
import Breadcrumb from "@/components/Breadcrumb";
import { CharacterProvider } from "@/providers/CharacterProvider";
import { SettingsProvider } from "@/providers/SettingsProvider";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Fabelis Dashboard",
  description: "A simple way to manage your Fabelis agent.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className + " dark"}>
        <SettingsProvider>
          <CharacterProvider>
            <SidebarProvider>
              <Sidebar />
              <SidebarInset className="w-[calc(100%_-_18rem)] max-h-[calc(100svh-theme(spacing.4))] overflow-y-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none] pt-16 pb-[9rem]">
                <header className="flex h-16 w-full items-center gap-2 fixed top-2 z-10 backdrop-blur-sm rounded-2xl">
                  <div className="flex items-center gap-2 px-4">
                    <SidebarTrigger className="-ml-1" />
                    <Separator orientation="vertical" className="mr-2 h-4" />
                    <Breadcrumb />
                  </div>
                </header>
                <div className="flex flex-1 flex-col strink-0 gap-4 p-4 pt-0 z-0">
                  {children}
                </div>
              </SidebarInset>
            </SidebarProvider>
          </CharacterProvider>
        </SettingsProvider>
      </body>
    </html>
  );
}
