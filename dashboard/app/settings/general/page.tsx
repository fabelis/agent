"use client";
import React, { useState, useEffect } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import {
  Globe,
  Terminal,
  MessageCircle,
  BookOpen,
  Send,
  Twitter,
  CheckCircle,
  Database,
  Cpu,
  Box,
  Save,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { useSettings, Settings } from "@/providers/SettingsProvider";
import { Separator } from "@/components/ui/separator";

const clientOptions = [
  { id: "api", name: "API", icon: Globe },
  { id: "cli", name: "CLI", icon: Terminal },
  { id: "discord", name: "Discord", icon: MessageCircle },
  { id: "storytelling", name: "Storytelling", icon: BookOpen },
  { id: "telegram", name: "Telegram", icon: Send },
  { id: "twitter", name: "Twitter", icon: Twitter },
  { id: "truth", name: "Truth", icon: CheckCircle },
];

const completionProviders = [
  "anthropic",
  "cohere",
  "gemini",
  "openai",
  "perplexity",
  "xai",
];
const embeddingProviders = ["local", "openai", "xai"];
const databaseProviders = ["local", "mongodb"];

export default function SettingsEditorWithFileSwitcher() {
  const { selectedSettings, saveSettings } = useSettings();
  const [changedSections, setChangedSections] = useState<Set<string>>(
    new Set()
  );
  const [editableSettings, setEditableSettings] = useState<Settings | null>(
    null
  );

  useEffect(() => {
    if (selectedSettings) {
      setEditableSettings(selectedSettings);
    }
  }, [selectedSettings]);

  useEffect(() => {
    if (!selectedSettings || !editableSettings) return;

    const newChangedSections = new Set<string>();
    if (
      JSON.stringify(selectedSettings.enabled_clients) !==
      JSON.stringify(editableSettings.enabled_clients)
    ) {
      newChangedSections.add("clients");
    }
    if (
      selectedSettings.completion_provider !==
        editableSettings.completion_provider ||
      selectedSettings.embedding_provider !==
        editableSettings.embedding_provider ||
      selectedSettings.db !== editableSettings.db
    ) {
      newChangedSections.add("providers");
    }
    setChangedSections(newChangedSections);
  }, [selectedSettings, editableSettings]);

  const handleClientToggle = (client: string) => {
    if (!editableSettings) return;
    setEditableSettings((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        enabled_clients: prev.enabled_clients.includes(client)
          ? prev.enabled_clients.filter((c) => c !== client)
          : [...prev.enabled_clients, client],
      };
    });
  };

  const handleSelectChange = (key: keyof Settings, value: string) => {
    if (!editableSettings) return;
    setEditableSettings((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        [key]: value,
      } as Settings;
    });
  };

  const handleSave = (section: string) => {
    if (!editableSettings) return;
    saveSettings(editableSettings);
    setChangedSections(
      new Set(Array.from(changedSections).filter((s) => s !== section))
    );
  };

  return (
    <div className="space-y-8">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Enabled Clients</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {clientOptions.map((client) => {
            const Icon = client.icon;
            const isEnabled =
              editableSettings?.enabled_clients?.includes(client.id) || false;
            return (
              <div
                key={client.id}
                className="flex items-center space-x-4 bg-secondary/20 p-4 rounded-lg"
              >
                <Icon className="h-6 w-6 text-primary" />
                <div className="flex-grow">
                  <Label htmlFor={client.id} className="text-sm font-medium">
                    {client.name}
                  </Label>
                </div>
                <Switch
                  id={client.id}
                  checked={isEnabled}
                  onCheckedChange={() => handleClientToggle(client.id)}
                />
              </div>
            );
          })}
        </div>
        <Button
          onClick={() => handleSave("clients")}
          className="w-full"
          disabled={!changedSections.has("clients")}
        >
          <Save className="h-4 w-4 mr-2" />
          Save Clients
          {changedSections.has("clients") && (
            <Badge variant="secondary" className="ml-2">
              Unsaved Changes
            </Badge>
          )}
        </Button>
      </div>
      <Separator />
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Providers</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="space-y-2">
            <Label
              htmlFor="completion_provider"
              className="flex items-center space-x-2"
            >
              <Cpu className="h-4 w-4" />
              <span>Completion Provider</span>
            </Label>
            <Select
              value={editableSettings?.completion_provider}
              onValueChange={(value) =>
                handleSelectChange("completion_provider", value)
              }
            >
              <SelectTrigger id="completion_provider">
                <SelectValue placeholder="Select completion provider" />
              </SelectTrigger>
              <SelectContent>
                {completionProviders.map((provider) => (
                  <SelectItem key={provider} value={provider}>
                    {provider}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label
              htmlFor="embedding_provider"
              className="flex items-center space-x-2"
            >
              <Box className="h-4 w-4" />
              <span>Embedding Provider</span>
            </Label>
            <Select
              value={editableSettings?.embedding_provider}
              onValueChange={(value) =>
                handleSelectChange("embedding_provider", value)
              }
            >
              <SelectTrigger id="embedding_provider">
                <SelectValue placeholder="Select embedding provider" />
              </SelectTrigger>
              <SelectContent>
                {embeddingProviders.map((provider) => (
                  <SelectItem key={provider} value={provider}>
                    {provider}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label htmlFor="db" className="flex items-center space-x-2">
              <Database className="h-4 w-4" />
              <span>Database Provider</span>
            </Label>
            <Select
              value={editableSettings?.db}
              onValueChange={(value) => handleSelectChange("db", value)}
            >
              <SelectTrigger id="db">
                <SelectValue placeholder="Select database provider" />
              </SelectTrigger>
              <SelectContent>
                {databaseProviders.map((provider) => (
                  <SelectItem key={provider} value={provider}>
                    {provider}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>
        <Button
          onClick={() => handleSave("providers")}
          className="w-full"
          disabled={!changedSections.has("providers")}
        >
          <Save className="h-4 w-4 mr-2" />
          Save Providers
          {changedSections.has("providers") && (
            <Badge variant="secondary" className="ml-2">
              Unsaved Changes
            </Badge>
          )}
        </Button>
      </div>
    </div>
  );
}
