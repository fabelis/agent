"use client";
import {
  useState,
  createContext,
  ReactNode,
  useContext,
  useEffect,
} from "react";

export interface CharacterContextProps {
  characters: Character[];
  selectedCharacter: Character | null;
  setSelectedCharacter: (character: Character) => void;
}

export type Character = {
  path_name: string;
  alias: string;
  bio: string;
  adjectives: string[];
  lore: string[];
  styles: string[];
  topics: string[];
  inspirations: string[];
};

export const CharacterContext = createContext<
  CharacterContextProps | undefined
>(undefined);

export interface CharacterProviderProps {
  children: ReactNode;
}

export const CharacterProvider: React.FC<CharacterProviderProps> = ({
  children,
}) => {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [selectedCharacter, setSelectedCharacter] = useState<Character | null>(
    null
  );

  useEffect(() => {
    fetch("/api/characters")
      .then((res) => {
        if (!res.ok) {
          throw new Error(`Failed to fetch characters: ${res.status}`);
        }
        return res.json();
      })
      .then((data) => {
        setCharacters(data);
      })
      .catch((error) => {
        setCharacters([]);
      });
  }, []);

  useEffect(() => {
    if (characters.length > 0 && !selectedCharacter) {
      setSelectedCharacter(characters[0]);
    }
  }, [characters]);

  const contextValue = {
    characters,
    selectedCharacter,
    setSelectedCharacter,
  };

  return (
    <CharacterContext.Provider value={contextValue}>
      {children}
    </CharacterContext.Provider>
  );
};

export const useCharacter = (): CharacterContextProps => {
  const context = useContext(CharacterContext);
  if (!context) {
    throw new Error("useCharacter must be used within a CharacterProvider");
  }
  return context;
};
