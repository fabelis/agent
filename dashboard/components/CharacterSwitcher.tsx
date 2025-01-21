"use client";

import * as React from "react";
import { Check, ChevronsUpDown, GalleryVerticalEnd } from "lucide-react";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useCharacter } from "@/providers/CharacterProvider";

const CharacterSwitcher = () => {
  const { selectedCharacter, characters, setSelectedCharacter } =
    useCharacter();

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                <GalleryVerticalEnd className="size-4" />
              </div>
              <div className="flex flex-col h-full gap-0.5 leading-none w-full">
                <span className="font-semibold">
                  {selectedCharacter?.alias}
                </span>
                <div className="w-full h-full flex-1 relative">
                  <span className="truncate text-xs text-muted-foreground min-w-0 flex-shrink absolute top-0 left-0 pr-4 max-w-full w-full">
                    {selectedCharacter?.path_name}
                  </span>
                </div>
              </div>
              <ChevronsUpDown className="ml-auto absolute right-0" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width]"
            align="start"
          >
            {characters.map((character, i) => (
              <DropdownMenuItem
                key={character.path_name}
                onSelect={() => setSelectedCharacter(character)}
              >
                {character.path_name}
                {character.path_name === selectedCharacter?.path_name && (
                  <Check className="ml-auto" />
                )}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
};

export default CharacterSwitcher;
