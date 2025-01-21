"use client";
import * as React from "react";
import {
  BookOpen,
  Bot,
  Command,
  MessagesSquare,
  PencilRuler,
  Settings2,
  SquareTerminal,
} from "lucide-react";
import {
  Sidebar as SidebarComponent,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { NavMain } from "./NavMain";
import Image from "next/image";
import FabelisLogo from "@/public/fabelis-logo.png";
import Link from "next/link";
import CharacterSwitcher from "./CharacterSwitcher";

const data = [
  {
    title: "Chat Room",
    url: "/chat",
    icon: MessagesSquare,
  },
  {
    title: "Character",
    url: "/character",
    icon: Bot,
  },
  {
    title: "Generation",
    url: "/gen",
    icon: PencilRuler,
    disabled: true,
    items: [
      {
        title: "TTS",
        url: "/gen/tts",
      },
      {
        title: "Art",
        url: "/gen/art",
      },
      {
        title: "Storytelling",
        url: "/gen/storytelling",
      },
    ],
  },
  {
    title: "Settings",
    url: "/settings",
    icon: Settings2,
    disabled: true,
    items: [
      {
        title: "Clients",
        url: "/settings?tab=clients",
      },
      {
        title: "Misc",
        url: "/settings?tab=misc",
      },
      {
        title: "Environment",
        url: "/settings?tab=environment",
      },
    ],
  },
];

export function Sidebar() {
  return (
    <SidebarComponent variant="inset" collapsible="icon">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <Link href="/">
                <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground overflow-clip">
                  <Image src={FabelisLogo} alt="Fabelis Logo" />
                </div>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-semibold">Fabelis AI</span>
                  <span className="truncate text-xs">Dashboard</span>
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={data} />
      </SidebarContent>
      <SidebarFooter>
        <CharacterSwitcher />
      </SidebarFooter>
    </SidebarComponent>
  );
}
