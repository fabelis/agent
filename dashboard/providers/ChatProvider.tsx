"use client";
import {
  useState,
  createContext,
  ReactNode,
  useContext,
  useEffect,
} from "react";
import { useCharacter } from "./CharacterProvider";

export interface ChatContextProps {
  messages: {
    [character_path_name: string]: Message[];
  };
  inProgress: boolean;
  addUserMessage: (content: string) => void;
  clearMessages: () => void;
  cancelLastMessage: () => void;
}

export const ChatContext = createContext<ChatContextProps | undefined>(
  undefined
);

export type Message = {
  role: "user" | "agent";
  content: string;
  inProgress?: boolean;
  canceled?: boolean;
  failed?: boolean;
};

export interface ChatProviderProps {
  children: ReactNode;
}

export const ChatProvider: React.FC<ChatProviderProps> = ({ children }) => {
  const { selectedCharacter } = useCharacter();
  const [messages, setMessages] = useState<{
    [character_path_name: string]: Message[];
  }>({});
  const [inProgress, setInProgress] = useState(false);

  const addUserMessage = (content: string) => {
    if (!selectedCharacter) return;
    let path_name = selectedCharacter.path_name;
    let _messages = messages[path_name] || [];
    setMessages((prevMessages) => ({
      ...prevMessages,
      [path_name]: [
        ...(prevMessages[path_name] || []),
        {
          role: "user",
          content,
          inProgress: true,
        },
      ],
    }));

    fetch("http://localhost:3001/prompt", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        path_name: path_name,
        prompt: content,
        history: (_messages || [])
          .filter((msg) => !msg.failed && !msg.canceled)
          .map((msg) => ({
            role: msg.role === "user" ? "user" : "assistant",
            content: msg.content,
          }))
          .slice(-10),
      }),
    })
      .then((response) => response.json())
      .then((data) => {
        setMessages((prevMessages) => {
          const messages = prevMessages[path_name] || [];
          const lastMessage = messages[messages.length - 1];

          if (lastMessage?.canceled) {
            return prevMessages;
          }

          return {
            ...prevMessages,
            [path_name]: [
              ...messages,
              {
                role: "agent",
                content: data.response,
              },
            ],
          };
        });
        setInProgress(false);
      })
      .catch((e) => {
        failLastMessage(path_name);
        setInProgress(false);
      });
  };

  const clearMessages = () => {
    if (!selectedCharacter) return;
    setMessages((prevMessages) => ({
      ...prevMessages,
      [selectedCharacter.path_name]: [],
    }));
  };

  const cancelLastMessage = () => {
    if (!selectedCharacter) return;
    setMessages((prevMessages) => {
      const messages = prevMessages[selectedCharacter.path_name] || [];
      const lastMessage = messages[messages.length - 1];
      if (lastMessage && lastMessage.inProgress) {
        lastMessage.canceled = true;
        lastMessage.inProgress = false;
      }
      return {
        ...prevMessages,
        [selectedCharacter.path_name]: messages,
      };
    });
  };

  const failLastMessage = (path_name: string) => {
    if (!selectedCharacter) return;
    setMessages((prevMessages) => {
      const messages = prevMessages[path_name] || [];
      const lastMessage = messages[messages.length - 1];
      if (lastMessage && lastMessage.inProgress) {
        lastMessage.inProgress = false;
        lastMessage.failed = true;
      }
      return {
        ...prevMessages,
        [path_name]: messages,
      };
    });
  };

  const completeLastMessage = (path_name: string) => {
    if (!selectedCharacter) return;
    setMessages((prevMessages) => {
      const messages = prevMessages[path_name] || [];
      const lastMessage = messages[messages.length - 1];
      if (lastMessage && lastMessage.inProgress) {
        lastMessage.inProgress = false;
      }
      return {
        ...prevMessages,
        [path_name]: messages,
      };
    });
  };

  const getSelectedMessages = () => {
    if (!selectedCharacter) return [];
    return messages[selectedCharacter.path_name] || [];
  };

  useEffect(() => {
    if (!selectedCharacter) return;
    const messages = getSelectedMessages();
    if (messages.length > 0) {
      const lastMessage = messages[messages.length - 1];
      setInProgress(lastMessage?.inProgress || false);
    } else {
      setInProgress(false);
    }
  }, [selectedCharacter, messages]);

  const contextValue = {
    messages,
    inProgress,
    addUserMessage,
    clearMessages,
    cancelLastMessage,
  };

  return (
    <ChatContext.Provider value={contextValue}>{children}</ChatContext.Provider>
  );
};

export const useChat = (): ChatContextProps => {
  const context = useContext(ChatContext);
  if (!context) {
    throw new Error("useChat must be used within a ChatProvider");
  }
  return context;
};
