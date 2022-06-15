import {
  Box, 
  Button, 
  Container, 
  Flex, 
  Heading, 
  LinkOverlay, 
  Text, 
  useDisclosure, 
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton, 
  InputGroup,
  InputLeftAddon,
  Input,
  InputRightAddon,
  Slider,
  SliderTrack,
  SliderFilledTrack,
  SliderThumb,
  Divider,
  Spacer,
  VStack,
  StackDivider,
  Badge
} from '@chakra-ui/react';
import React, { useState } from 'react';
import { colors } from '../../../constants/colors';
import { useStore as useWallet } from "../../../stores/wallet";

type Props = {
  shortVersion?: boolean
}

const ProjectList = (props: Props) => {
  const { wallet, isLogin} = useWallet();
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [sliderValue, setSliderValue] = useState(15)

  const vote = (voteId: any) => {
    
  }

  return (
    <section>
      <Container mt={'150px'} id="project-list">
        <Flex justifyContent={{ base: 'center', md: 'space-between' }} flexDirection={{ base: 'row', md: 'row' }} >
          <Heading lineHeight={'133%'} textAlign={{ base: 'center', md: 'start' }} fontWeight={700} color="gray.900" fontSize={'2xl'}> Project List </Heading>
        </Flex>
        <Flex mt={20} wrap={'wrap'} justifyContent={{ base: 'center', md: 'space-between' }} flexDirection={{ base: 'column', md: 'row' }}>
          <Box w={'30%'} border={'1px'} borderColor={colors.primary} p={10}>
            <Text fontSize={'2xl'}>Project Demo 1 - <Badge>Votes: 0</Badge></Text>
            <Text fontSize={'lg'} color={colors.primary} mt={5}>
                Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but occasionally circumstances occur in which toil and pain can procure him some great pleasure
            </Text>
            <Button mt={10} w={300} colorScheme={colors.primary}>
              Vote
            </Button>
          </Box>
          <Box w={'30%'} border={'1px'} borderColor={colors.primary} p={10}>
            <Text fontSize={'2xl'}>Project Demo 1 - <Badge>Votes: 0</Badge></Text>
            <Text fontSize={'lg'} color={colors.primary} mt={5}>
                Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but occasionally circumstances occur in which toil and pain can procure him some great pleasure
            </Text>
            <Button mt={10} w={300} colorScheme={colors.primary}>
              Vote
            </Button>
          </Box>
          <Box w={'30%'} border={'1px'} borderColor={colors.primary} p={10}>
            <Text fontSize={'2xl'}>Project Demo 1 - <Badge>Votes: 0</Badge></Text>
            <Text fontSize={'lg'} color={colors.primary} mt={5}>
                Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but occasionally circumstances occur in which toil and pain can procure him some great pleasure
            </Text>
            <Button mt={10} w={300} colorScheme={colors.primary}>
              Vote
            </Button>
          </Box>
        </Flex>
       
      </Container>
    </section>
  );
};

export default ProjectList;
