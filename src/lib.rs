/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/network-metrics
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use metricator::RateMetric;
use monotonic_time_rs::Millis;

#[cfg(feature = "log")]
use monotonic_time_rs::MillisDuration;

use std::fmt::Display;

pub struct MetricsInDirection {
    pub datagrams_per_second: f32,
    pub octets_per_second: f32,
}

impl Display for MetricsInDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} datagrams/s {} octets/s",
            self.datagrams_per_second, self.octets_per_second
        )
    }
}

pub struct CombinedMetrics {
    pub outgoing: MetricsInDirection,
    pub incoming: MetricsInDirection,
}

impl Display for CombinedMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metrics: out:\n{}, in:\n{}",
            self.outgoing, self.incoming
        )
    }
}

pub struct NetworkMetrics {
    in_datagrams_per_second: RateMetric,
    in_octets_per_second: RateMetric,
    out_datagrams_per_second: RateMetric,
    out_octets_per_second: RateMetric,
    #[cfg(feature = "log")]
    last_debug_metric_at: Millis,
    #[cfg(feature = "log")]
    debug_metric_duration: MillisDuration,
}

impl NetworkMetrics {
    pub fn new(now: Millis) -> Self {
        Self {
            in_datagrams_per_second: RateMetric::with_interval(now, 0.1),
            in_octets_per_second: RateMetric::with_interval(now, 0.1),

            out_datagrams_per_second: RateMetric::with_interval(now, 0.1),
            out_octets_per_second: RateMetric::with_interval(now, 0.1),
            #[cfg(feature = "log")]
            last_debug_metric_at: now,
            #[cfg(feature = "log")]
            debug_metric_duration: MillisDuration::from_millis(500),
        }
    }

    pub fn sent_datagrams(&mut self, datagrams: &Vec<Vec<u8>>) {
        for datagram in datagrams {
            self.out_octets_per_second.add(datagram.len() as u32)
        }
        self.out_datagrams_per_second.add(datagrams.len() as u32);
    }

    pub fn received_datagram(&mut self, datagram: &[u8]) {
        self.in_octets_per_second.add(datagram.len() as u32);
        self.in_datagrams_per_second.add(1);
    }

    pub fn update_metrics(&mut self, now: Millis) {
        self.in_datagrams_per_second.update(now);
        self.in_octets_per_second.update(now);
        self.out_datagrams_per_second.update(now);
        self.out_octets_per_second.update(now);

        #[cfg(feature = "log")]
        {
            use log::debug;
            if now - self.last_debug_metric_at > self.debug_metric_duration {
                self.last_debug_metric_at = now;
                debug!("metrics: {}", self.metrics())
            }
        }
    }

    pub fn metrics(&self) -> CombinedMetrics {
        CombinedMetrics {
            outgoing: MetricsInDirection {
                datagrams_per_second: self.out_datagrams_per_second.rate(),
                octets_per_second: self.out_octets_per_second.rate(),
            },
            incoming: MetricsInDirection {
                datagrams_per_second: self.in_datagrams_per_second.rate(),
                octets_per_second: self.in_octets_per_second.rate(),
            },
        }
    }

    pub fn in_datagrams_per_second(&self) -> f32 {
        self.in_datagrams_per_second.rate()
    }

    pub fn in_octets_per_second(&self) -> f32 {
        self.in_octets_per_second.rate()
    }

    pub fn out_datagrams_per_second(&self) -> f32 {
        self.out_datagrams_per_second.rate()
    }

    pub fn out_octets_per_second(&self) -> f32 {
        self.out_octets_per_second.rate()
    }
}
