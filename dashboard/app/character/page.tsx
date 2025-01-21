"use client";

import { useCharacter } from "@/providers/CharacterProvider";

export default function CharacterPage() {
  const { selectedCharacter } = useCharacter();
  return <></>;
}
