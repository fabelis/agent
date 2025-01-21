"use client";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { Character, useCharacter } from "@/providers/CharacterProvider";
import { Sparkles, X } from "lucide-react";

export default function CharacterPage() {
  const { selectedCharacter } = useCharacter();

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    // setCharacter({ ...selectedCharacter, [e.target.name]: e.target.value });
  };

  const handleArrayChange = (
    key: keyof Character,
    index: number,
    value: string
  ) => {
    // const newArray = [...(selectedCharacter[key] as string[])];
    // newArray[index] = value;
    // setCharacter({ ...selectedCharacter, [key]: newArray });
  };

  const handleAddItem = (key: keyof Character) => {
    // setCharacter({
    //   ...selectedCharacter,
    //   [key]: [...(selectedCharacter[key] as string[]), ""],
    // });
  };

  const handleRemoveItem = (key: keyof Character, index: number) => {
    // const newArray = [...(selectedCharacter[key] as string[])];
    // newArray.splice(index, 1);
    // setCharacter({ ...selectedCharacter, [key]: newArray });
  };

  const renderArrayEditor = (key: keyof Character, title: string) => (
    <div className="space-y-2">
      <div className="flex justify-between items-center">
        <h3 className="text-lg font-semibold">{title}</h3>
        <Button variant="outline" size="sm">
          <Sparkles className="h-4 w-4 mr-2" />
          Generate with AI
        </Button>
      </div>
      {selectedCharacter &&
        (selectedCharacter[key] as string[]).map((item, index) => (
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
      <Button onClick={() => handleAddItem(key)}>Add {title}</Button>
    </div>
  );

  return (
    <div className="bg-background flex flex-col">
      <main className="flex-grow flex">
        <Card className="w-full">
          <Tabs defaultValue="basic" className="w-full h-full flex flex-col">
            <TabsList className="grid w-full grid-cols-4">
              <TabsTrigger value="basic">Basic Info</TabsTrigger>
              <TabsTrigger value="lore">Lore</TabsTrigger>
              <TabsTrigger value="attributes">Attributes</TabsTrigger>
              <TabsTrigger value="inspiration">Inspiration</TabsTrigger>
            </TabsList>
            <CardContent className="flex-grow">
              <ScrollArea className="h-[calc(100vh-10rem)] pr-4">
                <TabsContent value="basic" className="space-y-4 mt-4">
                  <div className="space-y-2">
                    <label htmlFor="alias" className="text-sm font-medium">
                      Alias
                    </label>
                    <Input
                      id="alias"
                      name="alias"
                      value={selectedCharacter?.alias}
                      onChange={handleInputChange}
                    />
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between items-center">
                      <label htmlFor="bio" className="text-sm font-medium">
                        Bio
                      </label>
                      <Button variant="outline" size="sm">
                        <Sparkles className="h-4 w-4 mr-2" />
                        Generate Bio
                      </Button>
                    </div>
                    <Textarea
                      id="bio"
                      name="bio"
                      value={selectedCharacter?.bio}
                      onChange={handleInputChange}
                    />
                  </div>
                </TabsContent>
                <TabsContent value="lore" className="space-y-4 mt-4">
                  {renderArrayEditor("lore", "Lore")}
                </TabsContent>
                <TabsContent value="attributes" className="space-y-4 mt-4">
                  {renderArrayEditor("adjectives", "Adjectives")}
                  {renderArrayEditor("styles", "Styles")}
                  {renderArrayEditor("topics", "Topics")}
                </TabsContent>
                <TabsContent value="inspiration" className="space-y-4 mt-4">
                  {renderArrayEditor("inspirations", "Inspirations")}
                </TabsContent>
              </ScrollArea>
            </CardContent>
          </Tabs>
        </Card>
      </main>
    </div>
  );
}
