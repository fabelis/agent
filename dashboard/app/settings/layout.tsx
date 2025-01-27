"use client";
import { SettingsProvider } from "@/providers/SettingsProvider";
import { useState } from "react";
import { CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Plus, Check, Settings2, ChevronsUpDown } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { useSettings } from "@/providers/SettingsProvider";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { SidebarMenuButton } from "@/components/ui/sidebar";
import { Separator } from "@/components/ui/separator";
import { usePathname } from "next/navigation";

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <SettingsProvider>
      <LayoutBody>{children}</LayoutBody>
    </SettingsProvider>
  );
}

function LayoutBody({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const { settings, selectedSettings, setSelectedSettings, createSettings } =
    useSettings();
  const [newFileName, setNewFileName] = useState("");

  return (
    <ScrollArea className="w-full h-[calc(100vh-6rem)] rounded-xl border bg-card text-card-foreground shadow flex flex-col">
      <CardHeader className="!pb-0">
        <div className="flex justify-between items-center">
          <CardTitle className="text-2xl">
            {pathname === "/settings/general"
              ? "General"
              : pathname === "/settings/clients"
              ? "Clients"
              : ""}
          </CardTitle>
          <div className="flex items-center space-x-2">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <SidebarMenuButton className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground border h-10">
                  <div className="flex aspect-square size-6 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                    <Settings2 className="size-4" />
                  </div>
                  <div className="flex items-center h-full gap-0.5 leading-none w-full">
                    <span className="font-semibold">
                      {selectedSettings?.path_name}
                    </span>
                  </div>
                  <ChevronsUpDown className="ml-2" />
                </SidebarMenuButton>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                className="w-[--radix-dropdown-menu-trigger-width]"
                align="start"
              >
                {settings.map((setting) => (
                  <DropdownMenuItem
                    key={setting.path_name}
                    onSelect={() => setSelectedSettings(setting)}
                  >
                    {setting.path_name}
                    {setting.path_name === selectedSettings?.path_name && (
                      <Check className="ml-auto" />
                    )}
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
            <Dialog>
              <DialogTrigger asChild>
                <Button variant="outline" className="h-10 w-10">
                  <Plus className="h-4 w-4" />
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Create New Settings File</DialogTitle>
                </DialogHeader>
                <form
                  onSubmit={() => {
                    createSettings(newFileName);
                    setNewFileName("");
                  }}
                  className="flex items-center space-x-2"
                >
                  <Input
                    value={newFileName}
                    onChange={(e) => setNewFileName(e.target.value)}
                    placeholder="Enter file name"
                  />
                  <Button
                    disabled={
                      newFileName.length == 0 || !newFileName.endsWith(".json")
                    }
                  >
                    Create
                  </Button>
                </form>
              </DialogContent>
            </Dialog>
          </div>
        </div>
      </CardHeader>
      <div className="w-full px-6 my-4">
        <Separator />
      </div>
      <CardContent>
        <ScrollArea className="flex pr-4">{children}</ScrollArea>
      </CardContent>
    </ScrollArea>
  );
}
