import { Button, Container, Flex, Icon, useColorMode } from "@chakra-ui/react";
import Link from "next/link";
import { BsFillMoonStarsFill, BsFillSunFill } from "react-icons/bs";
import React from "react";

interface LayoutProps {
  children: React.ReactNode;
}

export default function Layout({children}: LayoutProps) {

  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <Container>
      <Flex position="fixed" top="1rem" right="1rem" align="center" justifyContent="center">
        <Link href="/" passHref>
          <Button as="a" variant="ghost" aria-label="Home" my={5} w="100%">
            Whitelist
          </Button>
        </Link>
        <Link href="/claim" passHref>
          <Button as="a" variant="ghost" aria-label="Home" my={5} w="100%">
            Claim
          </Button>
        </Link>
        <Button variant="outline" px={0} onClick={toggleColorMode}>
              <Icon
                as={colorMode === 'light' ? BsFillMoonStarsFill : BsFillSunFill}
              />
        </Button>
      </Flex>
      {children}
    </Container>
  );
}
