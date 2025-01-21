import { motion } from "motion/react";
import { Character, useCharacter } from "@/providers/CharacterProvider";
import { Message, useChat } from "@/providers/ChatProvider";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Button } from "./ui/button";
import { Clipboard } from "lucide-react";
import { useEffect, useRef } from "react";

const ChatMessages = () => {
  const { selectedCharacter } = useCharacter();
  const { messages, inProgress } = useChat();

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    if (messagesEndRef.current && containerRef.current) {
      messagesEndRef.current.scrollIntoView({
        behavior: "smooth",
      });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, inProgress]);

  return (
    <div
      ref={containerRef}
      className="max-w-full h-full flex flex-wrap flex-col space-y-2 relative"
    >
      {selectedCharacter &&
        messages[selectedCharacter.path_name] &&
        messages[selectedCharacter.path_name].map((message, index) =>
          message.role === "user" ? (
            <UserMessage key={index} message={message} />
          ) : (
            <AgentMessage key={index} content={message.content} />
          )
        )}
      {selectedCharacter && inProgress && (
        <AgentIsThinking character={selectedCharacter} />
      )}
      <div ref={messagesEndRef} />
    </div>
  );
};

const UserMessage = ({ message }: { message: Message }) => {
  return (
    <div className="w-full relative">
      <motion.div
        initial={{
          opacity: 0,
          scale: 0.9,
        }}
        animate={{
          opacity: 1,
          scale: 1,
        }}
        data-canceled={message.canceled}
        data-failed={message.failed}
        className="ml-auto max-w-[75%] w-fit bg-accent px-4 py-2 rounded-md data-[canceled=true]:!opacity-70 data-[canceled=true]:line-through data-[failed=true]:bg-destructive data-[failed=true]:!text-muted-foreground data-[failed=true]:!opacity-70 transition-all duration-300 flex flex-col"
      >
        <p className="break-words">{message.content}</p>
        {message.failed && (
          <motion.p
            initial={{
              opacity: 0,
              scale: 0.9,
            }}
            animate={{
              opacity: 1,
              scale: 1,
            }}
            className="text-xs text-destructive-foreground mt-1"
          >
            Failed To Send.
          </motion.p>
        )}
      </motion.div>
    </div>
  );
};

const AgentMessage = ({ content }: { content: string }) => {
  return (
    <div className="w-full flex flex-col space-y-1">
      <motion.div
        initial={{
          opacity: 0,
          scale: 0.9,
        }}
        animate={{
          opacity: 1,
          scale: 1,
        }}
        className="w-fit bg-primary px-4 py-2 rounded-md transition-all duration-300 flex flex-col"
      >
        <p className="break-words">{content}</p>
      </motion.div>
      <motion.div
        initial={{
          opacity: 0,
          y: 4,
        }}
        animate={{
          opacity: 1,
          y: 0,
        }}
        transition={{ delay: 0.25 }}
        className="w-fit flex space-x-2"
      >
        <Tooltip>
          <TooltipTrigger
            asChild
            onClick={() => navigator.clipboard.writeText(content)}
          >
            <Button variant="ghost" className="!p-0 aspect-square size-7">
              <Clipboard className="!size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent side="bottom">
            <p>Copy</p>
          </TooltipContent>
        </Tooltip>
      </motion.div>
    </div>
  );
};

const AgentIsThinking = ({ character }: { character: Character }) => {
  return (
    <div className="w-full">
      <motion.p
        initial={{
          opacity: 0,
          scale: 0.9,
        }}
        animate={{
          opacity: 1,
          scale: 1,
          transition: { delay: 0.25 },
        }}
        className="w-fit"
      >
        <Skeleton className="px-4 py-2 rounded-md">
          {character.alias} is thinking...
        </Skeleton>
      </motion.p>
    </div>
  );
};

export default ChatMessages;
