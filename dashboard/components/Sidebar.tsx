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
    url: "",
    icon: PencilRuler,
    disabled: true,
    items: [
      {
        title: "TTS",
        url: "",
      },
      {
        title: "Art",
        url: "",
      },
      {
        title: "Storytelling",
        url: "",
      },
    ],
  },
  {
    title: "Settings",
    url: "/settings/general",
    icon: Settings2,
    items: [
      {
        title: "General",
        url: "/settings/general",
      },
      {
        title: "Clients",
        url: "/settings/clients",
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
