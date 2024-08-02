mod typestate_doc_example;

use typestate::typestate;

/// Functor Typeclass
///
/// class Functor f where
/// fmap :: (a -> b) -> f a -> f b
///
/// Functor Laws
///
/// fmap id = id                   -- 1st functor law
/// fmap (g . f) = fmap g . fmap f -- 2nd functor law
///
/// -----------------------------------------------------------------------------------------------
///
/// Applicative Typeclass
///
/// class (Functor f) => Applicative f where
/// pure  :: a -> f a
/// (<*>) :: f (a -> b) -> f a -> f b
///
/// Applicative Laws
///
/// pure id <*> v = v                            -- Identity
/// pure f <*> pure x = pure (f x)               -- Homomorphism
/// u <*> pure y = pure ($ y) <*> u              -- Interchange
/// pure (.) <*> u <*> v <*> w = u <*> (v <*> w) -- Composition
///
/// -----------------------------------------------------------------------------------------------
///
/// Monad Typeclass
///
/// class Monad m where
/// (>>=)  :: m a -> (  a -> m b) -> m b
/// (>>)   :: m a ->  m b         -> m b
/// return ::   a                 -> m a
///
/// Monad Laws
///
/// return a >>= k                  =  k a
/// m        >>= return             =  m
/// m        >>= (\x -> k x >>= h)  =  (m >>= k) >>= h
///
#[typestate]
pub mod builder {
    use crate::llhd::module::LLHDModule;
    use crate::llhd_world::world::LLHDWorld;

    #[derive(Debug)]
    #[automaton]
    pub struct Flow {
        world: LLHDWorld,
    }

    #[state]
    pub struct Design;
    #[state]
    pub struct Technology;
    #[state]
    pub struct Synthesis;

    pub trait Design {
        fn load(module: LLHDModule) -> Technology;
        fn export(self);
    }

    pub trait Technology {
        fn constrain(self) -> Synthesis;
    }

    pub trait Synthesis {
        fn synthesize(self) -> Design;
    }

    impl DesignState for Flow<Design> {
        fn load(module: LLHDModule) -> Flow<Technology> {
            let world = LLHDWorld::new(module);
            Flow::<Technology> {
                world,
                state: Technology,
            }
        }

        fn export(self) {
            todo!()
        }
    }

    impl TechnologyState for Flow<Technology> {
        fn constrain(self) -> Flow<Synthesis> {
            todo!()
        }
    }

    impl SynthesisState for Flow<Synthesis> {
        fn synthesize(self) -> Flow<Design> {
            todo!()
        }
    }
}
