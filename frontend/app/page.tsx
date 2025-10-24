"use client";

import { Button } from "@/components/ui/button";
import { CampaignCard } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { useState, useEffect } from "react";
import { campaignApi } from "@/lib/api";
import { Campaign } from "@/lib/types";
import { motion } from "framer-motion";
import {
  Sparkles, Rocket, Shield, TrendingUp, Users, Heart,
  Zap, Globe, DollarSign, Star, ArrowRight, Play, Award
} from "lucide-react";
import { TextGenerateEffect } from "@/components/ui/text-generate-effect";
import { BlurFade } from "@/components/ui/blur-fade";
import { Marquee } from "@/components/ui/marquee";
import Link from "next/link";

export default function Home() {
  const [campaigns, setCampaigns] = useState<Campaign[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

  useEffect(() => {
    loadTrendingCampaigns();

    // Mouse tracking for interactive effects
    const handleMouseMove = (e: MouseEvent) => {
      setMousePosition({ x: e.clientX, y: e.clientY });
    };
    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, []);

  const loadTrendingCampaigns = async () => {
    try {
      const response = await campaignApi.getAll({});
      if (response.success && response.data) {
        const campaignData = Array.isArray(response.data) ? response.data : response.data.campaigns || [];
        setCampaigns(campaignData.slice(0, 6));
      }
    } catch (error) {
      console.error("Failed to load campaigns:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const features = [
    {
      icon: <Rocket className="w-7 h-7" />,
      title: "Launch Your Creator Hub",
      description: "Campaigns, memberships, events, podcast episodes, digital products — all orchestrated from one dashboard.",
      gradient: "from-purple-500 to-pink-500"
    },
    {
      icon: <Shield className="w-7 h-7" />,
      title: "Monetise Every Format",
      description: "Offer tiered subscriptions, paywalled articles, live experiences, and exclusive drops without stitching tools together.",
      gradient: "from-blue-500 to-cyan-500"
    },
    {
      icon: <Globe className="w-7 h-7" />,
      title: "Programmable Creator Feed",
      description: "Schedule highlights, pin announcements, and surface recommendations your community actually cares about.",
      gradient: "from-green-500 to-emerald-500"
    },
    {
      icon: <TrendingUp className="w-7 h-7" />,
      title: "Audience Intelligence",
      description: "Real-time analytics for revenue, retention, and content performance with cohort tracking and A/B insights.",
      gradient: "from-orange-500 to-red-500"
    },
  ];

  const stats = [
    { icon: <Users className="w-6 h-6" />, label: "Active Creators", value: "12,345+", gradient: "from-purple-500 to-pink-500" },
    { icon: <Heart className="w-6 h-6" />, label: "Supporters Reached", value: "3.8M", gradient: "from-red-500 to-pink-500" },
    { icon: <DollarSign className="w-6 h-6" />, label: "Recurring Revenue", value: "$12.7M", gradient: "from-blue-500 to-cyan-500" },
    { icon: <Award className="w-6 h-6" />, label: "Launch Success", value: "87%", gradient: "from-yellow-500 to-orange-500" },
  ];

  const testimonials = [
    {
      name: "Sarah Johnson",
      role: "Tech Educator",
      image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Sarah",
      quote: "I replaced four different tools with Fundify — campaigns, newsletters, memberships, even my live cohorts now live in one timeline."
    },
    {
      name: "Marcus Chen",
      role: "Game Developer",
      image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Marcus",
      quote: "The creator feed feels like Substack meets Patreon. Highlights, saved posts, and intelligent recommendations doubled my engagement overnight."
    },
    {
      name: "Emily Rodriguez",
      role: "Artist & Podcaster",
      image: "https://api.dicebear.com/7.x/avataaars/svg?seed=Emily",
      quote: "Fundify unlocked new income streams — premium episodes, events, merch drops — and the analytics show exactly what resonates."
    }
  ];

  return (
    <div className="flex flex-col overflow-hidden">
      {/* Animated Background - Monokai */}
      <div className="fixed inset-0 -z-10">
        <div className="absolute inset-0 bg-gradient-to-br from-pink-50 via-purple-50 to-cyan-50 dark:from-[#1E1E1E] dark:via-[#272822] dark:to-[#2D2A2E]" />
        <div
          className="absolute inset-0 opacity-20"
          style={{
            background: `radial-gradient(circle at ${mousePosition.x}px ${mousePosition.y}px, rgba(249, 38, 114, 0.2), transparent 50%)`
          }}
        />
        {/* Floating orbs - Monokai colors */}
        <div className="absolute top-20 left-10 w-72 h-72 bg-[#F92672]/20 rounded-full mix-blend-multiply filter blur-xl opacity-30 animate-blob dark:bg-[#F92672]/10" />
        <div className="absolute top-40 right-10 w-72 h-72 bg-[#E6DB74]/20 rounded-full mix-blend-multiply filter blur-xl opacity-30 animate-blob animation-delay-2000 dark:bg-[#E6DB74]/10" />
        <div className="absolute bottom-20 left-1/2 w-72 h-72 bg-[#66D9EF]/20 rounded-full mix-blend-multiply filter blur-xl opacity-30 animate-blob animation-delay-4000 dark:bg-[#66D9EF]/10" />
      </div>

      {/* Hero Section */}
      <section className="relative px-4 sm:px-6 lg:px-8 pt-24 pb-32">
        <div className="container mx-auto max-w-7xl">
          <div className="relative overflow-hidden rounded-[2.5rem] border border-white/10 dark:border-white/5 bg-white/70 dark:bg-[#161616]/75 backdrop-blur-2xl px-6 py-16 sm:px-12 md:px-16 shadow-[0_40px_120px_-40px_rgba(249,38,114,0.35)]">
            <div className="pointer-events-none absolute inset-0">
              <div className="absolute -top-32 left-1/2 h-[520px] w-[520px] -translate-x-1/2 rounded-full bg-[radial-gradient(circle,rgba(249,38,114,0.35)_0%,rgba(249,38,114,0)_60%)] blur-3xl" />
              <div className="absolute -bottom-20 right-16 h-[420px] w-[420px] rounded-full bg-[radial-gradient(circle,rgba(102,217,239,0.35)_0%,rgba(102,217,239,0)_65%)] blur-3xl" />
              <div className="absolute -left-10 top-1/2 h-[380px] w-[380px] -translate-y-1/2 rounded-full bg-[radial-gradient(circle,rgba(166,226,46,0.25)_0%,rgba(166,226,46,0)_65%)] blur-3xl" />
              <div className="absolute inset-0 bg-[radial-gradient(circle_at_top,rgba(255,255,255,0.45),rgba(255,255,255,0))]" />
            </div>

            <div className="relative text-center mb-16">
              {/* Badge */}
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.5 }}
              className="inline-flex items-center gap-2 px-4 py-2 bg-white/40 dark:bg-white/10 border border-white/40 text-sm font-semibold rounded-full shadow-soft"
            >
              <Sparkles className="w-4 h-4 text-[#F92672]" />
              <span className="text-gradient">
                Join 89,234+ creators running their business on Fundify
              </span>
            </motion.div>

            {/* Main Heading */}
            <TextGenerateEffect
              words="Ship campaigns. Stream content. Grow recurring revenue."
              className="text-5xl sm:text-6xl lg:text-7xl font-bold mb-6 leading-tight text-gradient-monokai"
            />

            {/* Subtitle */}
            <motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 1.5 }} // Increased delay to wait for the text generate effect
              className="text-xl text-muted-foreground mb-10 max-w-3xl mx-auto leading-relaxed"
            >
              Fundify is your command centre for the creator economy — launch campaigns, schedule posts, host live events, sell drops, and keep fans engaged with a programmable feed.
            </motion.p>

            {/* CTA Buttons */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.3 }}
              className="flex flex-col sm:flex-row gap-4 justify-center items-center"
            >
              <Link href="/campaigns/create">
                <Button size="lg" className="group relative overflow-hidden rounded-full px-8 py-6 text-lg bg-gradient-to-r from-[#F92672] via-[#AE81FF] to-[#66D9EF] shadow-[0_12px_50px_-20px_rgba(249,38,114,0.65)] hover:shadow-[0_18px_60px_-24px_rgba(174,129,255,0.55)]">
                  <span className="relative z-10 flex items-center gap-2">
                    Launch Your Creator Hub
                    <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
                  </span>
                  <div className="absolute inset-0 bg-gradient-to-r from-[#66D9EF] via-[#AE81FF] to-[#F92672] opacity-0 group-hover:opacity-100 transition-opacity" />
                </Button>
              </Link>
              <Link href="/feed">
                <Button size="lg" variant="glass" className="px-8 py-6 text-lg rounded-full bg-white/20 text-white dark:text-white border border-white/30">
                  <Play className="w-5 h-5 mr-2" />
                  Explore Creator Feed
                </Button>
              </Link>
            </motion.div>
            </div>

          {/* Stats Grid */}
          <motion.div
            initial={{ opacity: 0, y: 40 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.6, delay: 0.4 }}
            className="grid grid-cols-2 lg:grid-cols-4 gap-6 mt-20"
          >
            {stats.map((stat, index) => (
              <motion.article
                key={index}
                whileHover={{ y: -6 }}
                transition={{ type: "spring", stiffness: 260, damping: 20 }}
                className="group relative overflow-hidden rounded-3xl border border-white/20 dark:border-white/10 bg-white/70 dark:bg-[#1d1d1d]/75 backdrop-blur-xl p-6 shadow-[0_20px_60px_-35px_rgba(249,38,114,0.9)] transition-transform"
              >
                <div className="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-300 bg-gradient-to-br from-white/60 to-transparent dark:from-white/10" />
                <div className={`relative inline-flex p-3 rounded-xl bg-gradient-to-br ${stat.gradient} mb-4 text-white`}>
                  {stat.icon}
                </div>
                <div className="relative text-3xl font-bold text-foreground mb-1">
                  {stat.value}
                </div>
                <div className="relative text-sm text-muted-foreground">
                  {stat.label}
                </div>
              </motion.article>
            ))}
          </motion.div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="relative py-24 px-4 sm:px-6 lg:px-8">
        <div className="pointer-events-none absolute inset-0 -z-10 bg-gradient-to-br from-[#F92672]/8 via-transparent to-[#66D9EF]/10 dark:from-[#F92672]/15 dark:via-transparent dark:to-[#66D9EF]/12" />
        <div className="container mx-auto max-w-7xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold mb-4">
              <BlurFade delay={0.25} inView>
                <span className="text-gradient-monokai">
                  Everything You Need
                </span>
              </BlurFade>
            </h2>
            <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
              Powerful tools designed to help you succeed, from launch to scale
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {features.map((feature, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, y: 24 }}
                whileInView={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.6, delay: index * 0.08 }}
                viewport={{ once: true }}
                className="group relative overflow-hidden rounded-3xl border border-white/20 dark:border-white/10 bg-white/75 dark:bg-[#1A1A1A]/80 backdrop-blur-xl p-8 shadow-[0_25px_70px_-40px_rgba(174,129,255,0.65)] transition-transform hover:-translate-y-2"
              >
                <div className={`absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br ${feature.gradient} mix-blend-screen`} />
                <div className="relative inline-flex p-4 rounded-2xl bg-white/15 shadow-inner">
                  <div className="text-white drop-shadow-[0_12px_25px_rgba(0,0,0,0.12)]">
                    {feature.icon}
                  </div>
                </div>
                <h3 className="relative text-xl font-bold mt-6 mb-3 text-foreground">
                  {feature.title}
                </h3>
                <p className="relative text-sm leading-relaxed text-muted-foreground">
                  {feature.description}
                </p>
                <div className="relative mt-6 h-[2px] w-16 bg-gradient-to-r from-transparent via-white/70 to-transparent dark:via-white/30" />
              </motion.div>
            ))}
          </div>
        </div>
      </section>

      {/* Trending Campaigns */}
      <section className="py-24 px-4 sm:px-6 lg:px-8">
        <div className="container mx-auto max-w-7xl">
          <div className="flex justify-between items-center mb-12">
            <div>
                          <h2 className="text-4xl lg:text-5xl font-bold mb-4">
                            <BlurFade delay={0.25} inView>
                              <span className="bg-gradient-to-r from-blue-600 to-cyan-600 bg-clip-text text-transparent">
                                Trending Campaigns
                              </span>
                            </BlurFade>
                          </h2>              <p className="text-xl text-gray-600 dark:text-gray-400">
                Discover amazing projects from creators worldwide
              </p>
            </div>
            <Link href="/campaigns">
              <Button variant="outline" className="hidden sm:flex items-center gap-2">
                View All
                <ArrowRight className="w-4 h-4" />
              </Button>
            </Link>
          </div>

          {isLoading ? (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
              {[1, 2, 3].map((i) => (
                <Skeleton key={i} className="h-96 rounded-2xl" />
              ))}
            </div>
          ) : (
            <div className="relative flex h-full w-full flex-col items-center justify-center overflow-hidden rounded-lg">
              <Marquee pauseOnHover className="[--duration:120s]">
                {campaigns.map((campaign) => (
                  <div key={campaign.id} className="w-full max-w-sm mx-4">
                    <CampaignCard
                      title={campaign.title}
                      description={campaign.description}
                      imageUrl={campaign.imageUrl}
                      currentAmount={campaign.currentAmount}
                      goal={campaign.goal}
                      slug={campaign.slug}
                      category={campaign.category}
                      daysRemaining={Math.ceil((new Date(campaign.endDate).getTime() - new Date().getTime()) / (1000 * 60 * 60 * 24))}
                      backers={campaign.backers}
                    />
                  </div>
                ))}
              </Marquee>
            </div>
          )}
        </div>
      </section>

      {/* Testimonials */}
      <section className="py-24 px-4 sm:px-6 lg:px-8 bg-gradient-to-br from-purple-50 to-pink-50 dark:from-gray-900 dark:to-gray-800">
        <div className="container mx-auto max-w-7xl">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold mb-4">
              <BlurFade delay={0.25} inView>
                <span className="bg-gradient-to-r from-purple-600 to-pink-600 bg-clip-text text-transparent">
                  Loved by Creators
                </span>
              </BlurFade>
            </h2>
            <p className="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto">
              Join thousands of successful creators who chose Fundify
            </p>
          </div>

          <div className="relative flex h-full w-full flex-col items-center justify-center overflow-hidden rounded-lg py-12">
            <Marquee pauseOnHover className="[--duration:60s]">
              {testimonials.map((testimonial, index) => (
                <div
                  key={index}
                  className="bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg w-[350px] mx-4 border border-gray-200 dark:border-gray-700"
                >
                  <div className="flex items-center gap-1 mb-4">
                    {[...Array(5)].map((_, i) => (
                      <Star key={i} className="w-5 h-5 fill-yellow-400 text-yellow-400" />
                    ))}
                  </div>
                  <p className="text-gray-700 dark:text-gray-300 mb-6 leading-relaxed italic">
                    &quot;{testimonial.quote}&quot;
                  </p>
                  <div className="flex items-center gap-4">
                    <img
                      src={testimonial.image}
                      alt={testimonial.name}
                      className="w-12 h-12 rounded-full"
                    />
                    <div>
                      <div className="font-bold text-gray-900 dark:text-white">
                        {testimonial.name}
                      </div>
                      <div className="text-sm text-gray-600 dark:text-gray-400">
                        {testimonial.role}
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </Marquee>
          </div>
        </div>
      </section>

      {/* Final CTA */}
      <section className="py-24 px-4 sm:px-6 lg:px-8">
        <div className="container mx-auto max-w-4xl">
          <div className="relative bg-gradient-to-r from-purple-600 to-pink-600 rounded-3xl p-12 text-center overflow-hidden">
            <div className="absolute inset-0 bg-black/10" />
            <div className="relative z-10">
              <h2 className="text-4xl lg:text-5xl font-bold text-white mb-6">
                Ready to Start Your Journey?
              </h2>
              <p className="text-xl text-white/90 mb-8 max-w-2xl mx-auto">
                Join thousands of creators who are already funding their dreams on Fundify
              </p>
              <Link href="/register">
                <Button size="lg" className="bg-white text-purple-600 hover:bg-gray-100 px-8 py-6 text-lg rounded-full shadow-lg">
                  <Zap className="w-5 h-5 mr-2" />
                  Get Started Free
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
