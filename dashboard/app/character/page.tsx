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
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import { useToast } from "@/hooks/use-toast";
import { ProgressSpinner } from "@/components/ui/progress-spinner";
import { Checkbox } from "@/components/ui/checkbox";

export default function CharacterPage() {
  const { selectedCharacter, saveCharacter } = useCharacter();
  const [changedSections, setChangedSections] = useState<Set<string>>(
    new Set()
  );
  const [editableCharacter, setEditableCharacter] = useState<Character | null>(
    null
  );
  const [aiEditedCharacter, setAiEditedCharacter] = useState<Character | null>(
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
          {aiEditedCharacter !== null &&
          editableCharacter?.[key] !== aiEditedCharacter?.[key] ? (
            renderAiChangesButtons()
          ) : (
            <GenerateButton
              title={title}
              field={key}
              editableCharacter={editableCharacter}
              aiEditedCharacter={aiEditedCharacter}
              onSuccess={setAiEditedCharacter}
            />
          )}
        </div>
      </div>
      {editableCharacter &&
        (aiEditedCharacter && editableCharacter[key] !== aiEditedCharacter[key]
          ? (aiEditedCharacter[key] as string[]).map((item, index) => (
              <div key={index} className="flex items-center space-x-2">
                <div className="animate-pulse w-full">
                  <Input
                    disabled
                    value={item}
                    onChange={(e) =>
                      handleArrayChange(key, index, e.target.value)
                    }
                    className={`flex-grow ${
                      !editableCharacter[key][index] ||
                      editableCharacter[key][index] !== item
                        ? "border-2 border-primary disabled:!opacity-100"
                        : ""
                    }`}
                  />
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  disabled
                  onClick={() => handleRemoveItem(key, index)}
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ))
          : (editableCharacter[key] as string[]).map((item, index) => (
              <div key={index} className="flex items-center space-x-2">
                <Input
                  value={item}
                  onChange={(e) =>
                    handleArrayChange(key, index, e.target.value)
                  }
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
            )))}
    </div>
  );

  const renderAiChangesButtons = () => {
    return (
      <div className="flex items-center gap-2 flex-wrap">
        <Button
          onClick={() => {
            if (!editableCharacter || !aiEditedCharacter) return;
            setEditableCharacter(aiEditedCharacter);
            setAiEditedCharacter(null);
          }}
          size="sm"
        >
          <Sparkles className="h-4 w-4 mr-2" />
          Accept Changes
        </Button>
        <Button
          onClick={() => {
            if (!editableCharacter || !aiEditedCharacter) return;
            setAiEditedCharacter(null);
          }}
          variant="secondary"
          size="sm"
        >
          <Sparkles className="h-4 w-4 mr-2" />
          Decline Changes
        </Button>
      </div>
    );
  };

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

const GenerateButton = ({
  title,
  field,
  editableCharacter,
  aiEditedCharacter,
  onSuccess,
}: {
  title: string;
  field: keyof Character;
  editableCharacter: Character | null;
  aiEditedCharacter: Character | null;
  onSuccess: (character: Character) => void;
}) => {
  const [isGenerating, setIsGenerating] = useState(false);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline" size="sm">
          <Sparkles className="h-4 w-4 mr-2" />
          Generate with AI
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px] lg:max-w-6xl">
        <DialogHeader>
          <DialogTitle>Generate {title} Content</DialogTitle>
          <DialogDescription>
            Use AI to generate new content for this field. Review and edit
            before saving.
          </DialogDescription>
        </DialogHeader>
        <form
          className="flex flex-col gap-4"
          onSubmit={async (e) => {
            e.preventDefault();
            const formData = new FormData(e.currentTarget);
            const prompt = formData.get("prompt") as string;
            const keepCurrent = formData.get("keepCurrent") === "on";
            const numFields =
              parseInt(formData.get("numFields") as string) || 1;

            if (!editableCharacter || aiEditedCharacter || !prompt) return;

            console.log({
              character_data: editableCharacter,
              prompt,
              field,
              keep_current: keepCurrent,
              num_fields: numFields,
            });

            setIsGenerating(true);
            try {
              const response = await fetch(
                "http://localhost:3001/character/gen",
                {
                  method: "POST",
                  headers: {
                    "Content-Type": "application/json",
                  },
                  body: JSON.stringify({
                    character_data: editableCharacter,
                    prompt,
                    field,
                    keep_current: keepCurrent,
                    num_fields: numFields,
                  }),
                }
              );

              if (!response.ok) {
                throw new Error("Failed to generate content");
              }

              const data = await response.json();

              if (data.error) {
                throw new Error(data.error);
              }

              const newAiCharacter = {
                ...editableCharacter,
                [field]: data.content,
              };

              onSuccess(newAiCharacter);
            } catch (error) {
            } finally {
              setIsGenerating(false);
            }
          }}
        >
          <div className="flex flex-col gap-2">
            <Label htmlFor="prompt">Prompt</Label>
            <Input
              id="prompt"
              name="prompt"
              placeholder="Enter prompt for AI generation"
              className="col-span-3"
            />
          </div>
          <div className="flex flex-col gap-2">
            <Label htmlFor="numFields">Number of Fields</Label>
            <Input
              id="numFields"
              name="numFields"
              type="number"
              min="1"
              defaultValue="1"
              className="col-span-3"
            />
          </div>
          <div className="flex items-center gap-2">
            <Checkbox id="keepCurrent" name="keepCurrent" />
            <Label htmlFor="keepCurrent">Keep current fields</Label>
          </div>
          <DialogFooter>
            <Button disabled={isGenerating} type="submit">
              {isGenerating ? <ProgressSpinner /> : "Generate"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
