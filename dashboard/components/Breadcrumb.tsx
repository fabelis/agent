"use client";
import {
  Breadcrumb as BreadcrumbComponent,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { useCharacter } from "@/providers/CharacterProvider";
import { usePathname } from "next/navigation";

const data: any = {
  "/": "Home",
  "/chat": "Chat Room",
  "/character": "Character Editor",
};

const Breadcrumb = () => {
  const pathname = usePathname();
  const { selectedCharacter } = useCharacter();

  return (
    <BreadcrumbComponent>
      <BreadcrumbList>
        <BreadcrumbItem className="hidden md:block">
          <BreadcrumbLink href={pathname}>{data?.[pathname]}</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbData />
      </BreadcrumbList>
    </BreadcrumbComponent>
  );
};

const BreadcrumbData = () => {
  const pathname = usePathname();
  const { selectedCharacter } = useCharacter();

  return (
    <>
      {pathname == "/chat" ||
        ("/character" && (
          <>
            <BreadcrumbSeparator className="hidden md:block" />
            <BreadcrumbItem>
              <BreadcrumbPage>{selectedCharacter?.alias}</BreadcrumbPage>
            </BreadcrumbItem>
          </>
        ))}
    </>
  );
};

export default Breadcrumb;
