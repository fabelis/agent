"use client";
import ChatBox from "@/components/ChatBox";
import ChatMessages from "@/components/ChatMessages";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useCharacter } from "@/providers/CharacterProvider";
import { ArrowUp } from "lucide-react";

export default function ChatPage() {
  const { selectedCharacter } = useCharacter();
  return (
    <div className="flex min-w-0 flex-col h-full pb-[9rem]">
      <ChatMessages />
      <ChatBox />
    </div>
  );
}
