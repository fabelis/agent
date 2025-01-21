"use client";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { Character, useCharacter } from "@/providers/CharacterProvider";
import { Sparkles, X, Save, Plus } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { useState, useEffect } from "react";

export default function CharacterPage() {
  const { selectedCharacter, saveCharacter } = useCharacter();
  const [changedSections, setChangedSections] = useState<Set<string>>(
    new Set()
  );
  const [editableCharacter, setEditableCharacter] = useState<Character | null>(
    null
  );

  useEffect(() => {
    if (selectedCharacter) {
      setEditableCharacter(selectedCharacter);
    }
  }, [selectedCharacter]);

  useEffect(() => {
    if (!selectedCharacter || !editableCharacter) return;

    const newChangedSections = new Set<string>();
    if (
      selectedCharacter.alias !== editableCharacter.alias ||
      selectedCharacter.bio !== editableCharacter.bio
    ) {
      newChangedSections.add("basic");
    }
    if (
      JSON.stringify(selectedCharacter.lore) !==
      JSON.stringify(editableCharacter.lore)
    ) {
      newChangedSections.add("lore");
    }
    if (
      JSON.stringify(selectedCharacter.adjectives) !==
        JSON.stringify(editableCharacter.adjectives) ||
      JSON.stringify(selectedCharacter.styles) !==
        JSON.stringify(editableCharacter.styles) ||
      JSON.stringify(selectedCharacter.topics) !==
        JSON.stringify(editableCharacter.topics)
    ) {
      newChangedSections.add("attributes");
    }
    if (
      JSON.stringify(selectedCharacter.inspirations) !==
      JSON.stringify(editableCharacter.inspirations)
    ) {
      newChangedSections.add("inspiration");
    }
    setChangedSections(newChangedSections);
  }, [selectedCharacter, editableCharacter]);

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    if (!editableCharacter) return;
    setEditableCharacter({
      ...editableCharacter,
      [e.target.name]: e.target.value,
    });
  };

  const handleArrayChange = (
    key: keyof Character,
    index: number,
    value: string
  ) => {
    if (!editableCharacter) return;
    setEditableCharacter({
      ...editableCharacter,
      [key]: (editableCharacter[key] as string[]).map((item, i) =>
        i === index ? value : item
      ),
    });
  };

  const handleAddItem = (key: keyof Character) => {
    if (!editableCharacter) return;
    setEditableCharacter({
      ...editableCharacter,
      [key]: [...(editableCharacter[key] as string[]), ""],
    });
  };

  const handleRemoveItem = (key: keyof Character, index: number) => {
    if (!editableCharacter) return;
    const newArray = [...(editableCharacter[key] as string[])];
    newArray.splice(index, 1);
    setEditableCharacter({
      ...editableCharacter,
      [key]: newArray,
    });
  };

  const handleSave = (section: string) => {
    if (!editableCharacter) return;
    saveCharacter(editableCharacter);
    setChangedSections(
      new Set(Array.from(changedSections).filter((s) => s !== section))
    );
  };

  const renderArrayEditor = (
    key: keyof Character,
    title: string,
    description: string
  ) => (
    <div className="space-y-2">
      <div className="flex justify-between items-end">
        <div>
          <h3 className="text-lg font-semibold">{title}</h3>
          <p className="text-sm text-muted-foreground mb-2">{description}</p>
        </div>
        <div className="flex gap-2 items-center">
          <Button
            variant="secondary"
            className="!p-0 aspect-square size-8"
            onClick={() => handleAddItem(key)}
          >
            <Plus />
          </Button>
          <Button variant="outline" size="sm">
            <Sparkles className="h-4 w-4 mr-2" />
            Generate with AI
          </Button>
        </div>
      </div>
      {editableCharacter &&
        (editableCharacter[key] as string[]).map((item, index) => (
          <div key={index} className="flex items-center space-x-2">
            <Input
              value={item}
              onChange={(e) => handleArrayChange(key, index, e.target.value)}
              className="flex-grow"
            />
            <Button
              variant="ghost"
              size="icon"
              onClick={() => handleRemoveItem(key, index)}
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
        ))}
    </div>
  );

  const renderSaveButton = (section: string) => (
    <Button
      onClick={() => handleSave(section)}
      className="w-full mt-4"
      disabled={!changedSections.has(section)}
    >
      <Save className="h-4 w-4 mr-2" />
      Save {section.charAt(0).toUpperCase() + section.slice(1)}
      {changedSections.has(section) && (
        <Badge variant="secondary" className="ml-2">
          Unsaved Changes
        </Badge>
      )}
    </Button>
  );

  return (
    <ScrollArea className="w-full h-[calc(100vh-6rem)] rounded-xl border bg-card text-card-foreground shadow">
      <Tabs defaultValue="basic" className="w-full flex flex-col">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="basic">
            Basic Info
            {changedSections.has("basic") && (
              <Badge variant="secondary" className="ml-2">
                *
              </Badge>
            )}
          </TabsTrigger>
          <TabsTrigger value="lore">
            Lore
            {changedSections.has("lore") && (
              <Badge variant="secondary" className="ml-2">
                *
              </Badge>
            )}
          </TabsTrigger>
          <TabsTrigger value="attributes">
            Attributes
            {changedSections.has("attributes") && (
              <Badge variant="secondary" className="ml-2">
                *
              </Badge>
            )}
          </TabsTrigger>
          <TabsTrigger value="inspiration">
            Inspiration
            {changedSections.has("inspiration") && (
              <Badge variant="secondary" className="ml-2">
                *
              </Badge>
            )}
          </TabsTrigger>
        </TabsList>
        <CardContent className="flex-grow">
          <ScrollArea className="h-full pr-4">
            <TabsContent value="basic" className="space-y-4 mt-4">
              <div className="space-y-2">
                <div className="flex justify-between items-end">
                  <div>
                    <label htmlFor="alias" className="font-semibold">
                      Alias
                    </label>
                    <p className="text-sm text-muted-foreground mb-2">
                      The name your character goes by
                    </p>
                  </div>
                  <Button variant="outline" size="sm">
                    <Sparkles className="h-4 w-4 mr-2" />
                    Generate Alias
                  </Button>
                </div>
                <Input
                  id="alias"
                  name="alias"
                  value={editableCharacter?.alias}
                  onChange={handleInputChange}
                />
              </div>
              <div className="space-y-2">
                <div className="flex justify-between items-end">
                  <div>
                    <label htmlFor="bio" className="font-semibold">
                      Bio
                    </label>
                    <p className="text-sm text-muted-foreground mb-2">
                      A brief description of your character's background and
                      personality
                    </p>
                  </div>
                  <Button variant="outline" size="sm">
                    <Sparkles className="h-4 w-4 mr-2" />
                    Generate Bio
                  </Button>
                </div>
                <Textarea
                  id="bio"
                  name="bio"
                  value={editableCharacter?.bio}
                  onChange={handleInputChange}
                />
              </div>
              {renderSaveButton("basic")}
            </TabsContent>
            <TabsContent value="lore" className="space-y-4 mt-4">
              {renderArrayEditor(
                "lore",
                "Lore",
                "Key events and background stories that shape your character's history and motivations"
              )}
              {renderSaveButton("lore")}
            </TabsContent>
            <TabsContent value="attributes" className="space-y-4 mt-4">
              {renderArrayEditor(
                "adjectives",
                "Adjectives",
                "Personality traits and characteristics that define your character's behavior"
              )}
              {renderArrayEditor(
                "styles",
                "Styles",
                "Writing styles and tones that influence how your character communicates"
              )}
              {renderArrayEditor(
                "topics",
                "Topics",
                "Subjects and themes your character is knowledgeable about or interested in"
              )}
              {renderSaveButton("attributes")}
            </TabsContent>
            <TabsContent value="inspiration" className="space-y-4 mt-4">
              {renderArrayEditor(
                "inspirations",
                "Inspirations",
                "References and influences that helped shape your character's development"
              )}
              {renderSaveButton("inspiration")}
            </TabsContent>
          </ScrollArea>
        </CardContent>
      </Tabs>
    </ScrollArea>
  );
}
